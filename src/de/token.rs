pub mod byte;

use std::{io, str::FromStr};

use crate::error::{NeverFail, SyntaxError};

use super::position::{PosRange, Position};

pub trait TokenizerExt {
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>>;
    fn find(&mut self) -> crate::Result<Option<(Position, u8)>>;

    fn eat_whitespace(&mut self) -> crate::Result<Option<(Position, u8)>> {
        while let Some((pos, c)) = self.eat()? {
            if !c.is_ascii_whitespace() {
                return Ok(Some((pos, c)));
            }
        }
        Ok(None)
    }

    fn skip_whitespace(&mut self) -> crate::Result<Option<(Position, u8)>> {
        while let Some((pos, c)) = self.find()? {
            if c.is_ascii_whitespace() {
                self.eat()?;
            } else {
                return Ok(Some((pos, c)));
            }
        }
        Ok(None)
    }

    fn fold_token<F: FnMut(&[u8], u8) -> bool>(&mut self, mut f: F) -> crate::Result<(Option<PosRange>, Vec<u8>)> {
        let (mut range, mut buff) = (None, Vec::new());
        while let Some((pos, c)) = self.find()? {
            range = if range.is_none() { Some((pos, pos)) } else { range };
            if f(&buff, c) {
                let (p, c) = self.eat()?.ok_or(NeverFail::EatAfterFind)?;
                range.as_mut().map(|(_, t)| *t = p);
                buff.push(c);
            } else {
                break;
            }
        }
        Ok((range, buff))
    }

    fn parse_ident<T>(&mut self, ident: &[u8], value: T) -> crate::Result<T> {
        let mut iter = ident.into_iter();
        let (p, parsed) = self.fold_token(|_, c| iter.next().map_or(false, |&i| i == c))?;
        match (p, iter.next().is_none() && parsed.len() == ident.len()) {
            (_, true) => Ok(value),
            (Some(pos), false) => Err(SyntaxError::UnexpectedIdent { pos, expected: ident.into(), found: parsed })?,
            (None, false) => Err(SyntaxError::EofWhileParsingIdent)?,
        }
    }

    fn parse_string(&mut self) -> crate::Result<Vec<u8>> {
        let mut buff = Vec::new();
        match self.eat_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingString)? {
            (_, b'"') => {
                self.parse_string_content(&mut buff)?;
                match self.eat()?.ok_or(SyntaxError::EofWhileEndParsingString)? {
                    (_, b'"') => Ok(buff),
                    (pos, found) => Err(SyntaxError::UnexpectedTokenWhileEndParsingString { pos, found })?,
                }
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingString { pos, found })?,
        }
    }

    fn parse_string_content(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
        while let Some((pos, found)) = self.find()? {
            match found {
                b'\\' => self.parse_escape_sequence(buff)?,
                b'"' => return Ok(()),
                c if c.is_ascii_control() => Err(SyntaxError::ControlCharacterWhileParsingString { pos, c })?,
                _ => buff.push(self.eat()?.ok_or(NeverFail::EatAfterFind)?.1),
            }
        }
        Err(SyntaxError::EofWhileEndParsingString)? // TODO contain parsed string?
    }

    fn parse_escape_sequence(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
        match self.eat()?.ok_or(SyntaxError::EofWhileParsingEscapeSequence)? {
            (_, b'\\') => match self.eat()?.ok_or(SyntaxError::EofWhileParsingEscapeSequence)? {
                (_, b'"') => Ok(buff.push(b'"')),
                (_, b'\\') => Ok(buff.push(b'\\')),
                (_, b'/') => Ok(buff.push(b'/')),
                (_, b'b') => Ok(buff.push(b'\x08')),
                (_, b'f') => Ok(buff.push(b'\x0C')),
                (_, b'n') => Ok(buff.push(b'\n')),
                (_, b'r') => Ok(buff.push(b'\r')),
                (_, b't') => Ok(buff.push(b'\t')),
                (_, b'u') => Ok(self.parse_unicode(buff)?),
                (pos, found) => Err(SyntaxError::InvalidEscapeSequence { pos, found })?,
            },
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingEscapeSequence { pos, found })?,
        }
    }

    fn parse_unicode(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
        let mut hex: u32 = 0;
        for i in 0..4 {
            match self.eat()?.ok_or(SyntaxError::EofWhileParsingEscapeSequence)? {
                (_, c @ b'0'..=b'9') => hex += ((c - b'0' + 0) as u32) << 4 * (3 - i),
                (_, c @ b'a'..=b'f') => hex += ((c - b'a' + 10) as u32) << 4 * (3 - i),
                (_, c @ b'A'..=b'F') => hex += ((c - b'A' + 10) as u32) << 4 * (3 - i),
                (pos, found) => return Err(SyntaxError::InvalidUnicodeEscape { pos, found })?,
            }
        }
        let ch = unsafe { char::from_u32_unchecked(hex) }; // TODO maybe safe
        Ok(buff.extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes()))
    }

    fn parse_number<T: FromStr>(&mut self) -> crate::Result<T> {
        let mut buff = Vec::new(); // TODO performance optimization (do not use string buffer)
        let (pos, _) = self.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingNumber)?;
        match self.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingNumber)? {
            (_, b'-') => buff.push(self.eat()?.ok_or(NeverFail::EatAfterFind)?.1),
            (pos, b'+') => Err(SyntaxError::InvalidLeadingPlus { pos })?,
            _ => (),
        }
        match self.eat()?.ok_or(SyntaxError::EofWhileParsingNumber)? {
            (_, c @ b'0') => match self.find()? {
                Some((pos, b'0'..=b'9')) => Err(SyntaxError::InvalidLeadingZeros { pos })?,
                _ => buff.push(c),
            },
            (_, c @ b'1'..=b'9') => {
                buff.push(c);
                buff.extend_from_slice(&self.fold_token(|_, c| matches!(c, b'0'..=b'9'))?.1);
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingNumber { pos, found })?,
        }
        match self.find()? {
            Some((_, b'.')) => {
                buff.push(self.eat()?.ok_or(NeverFail::EatAfterFind)?.1);
                match self.find()?.ok_or(SyntaxError::EofWhileStartParsingFraction)? {
                    (_, b'0'..=b'9') => buff.extend_from_slice(&self.fold_token(|_, c| matches!(c, b'0'..=b'9'))?.1),
                    (pos, found) => Err(SyntaxError::MissingFraction { pos, found })?,
                }
            }
            _ => (),
        }
        match self.find()? {
            Some((_, b'e' | b'E')) => {
                buff.push(self.eat()?.ok_or(NeverFail::EatAfterFind)?.1);
                match self.eat()?.ok_or(SyntaxError::EofWhileStartParsingExponent)? {
                    (_, c @ (b'+' | b'-' | b'0'..=b'9')) => buff.push(c),
                    (pos, found) => Err(SyntaxError::MissingExponent { pos, found })?,
                }
                buff.extend_from_slice(&self.fold_token(|_, c| matches!(c, b'0'..=b'9'))?.1);
            }
            _ => (),
        }
        let representation = String::from_utf8(buff)?;
        Ok(representation.parse().or(Err(SyntaxError::InvalidNumber { pos, rep: representation }))?)
    }
}

pub struct RowColIterator<I> {
    iter: I,
    row: usize,
    col: usize,
}
impl<I> RowColIterator<I> {
    pub fn new(iter: I) -> Self {
        RowColIterator { iter, row: 0, col: 0 }
    }

    pub fn pos(&self) -> Position {
        (self.row, self.col)
    }
}
impl<I> Iterator for RowColIterator<I>
where
    I: Iterator<Item = io::Result<u8>>,
{
    type Item = (Position, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|c| {
            let pos = self.pos();
            match c {
                Ok(b'\n') => {
                    self.row += 1;
                    self.col = 0;
                }
                Ok(_) => self.col += 1,
                Err(_) => {}
            }
            (pos, c)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Read};

    use super::*;

    #[test]
    fn behavior_row_col_iterator() {
        // [
        //   "foo",
        //   "bar",
        //   "baz"
        // ]
        let raw = vec!["[", r#"  "foo","#, r#"  "bar","#, r#"  "baz""#, "]"].join("\n");
        let reader = BufReader::new(raw.as_bytes());
        let mut iter = RowColIterator::new(reader.bytes());

        assert!(matches!(iter.next(), Some(((0, 0), Ok(b'[')))));
        assert!(matches!(iter.next(), Some(((0, 1), Ok(b'\n')))));

        assert!(matches!(iter.next(), Some(((1, 0), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((1, 1), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((1, 2), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((1, 3), Ok(b'f')))));
        assert!(matches!(iter.next(), Some(((1, 4), Ok(b'o')))));
        assert!(matches!(iter.next(), Some(((1, 5), Ok(b'o')))));
        assert!(matches!(iter.next(), Some(((1, 6), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((1, 7), Ok(b',')))));
        assert!(matches!(iter.next(), Some(((1, 8), Ok(b'\n')))));

        assert!(matches!(iter.next(), Some(((2, 0), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((2, 1), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((2, 2), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((2, 3), Ok(b'b')))));
        assert!(matches!(iter.next(), Some(((2, 4), Ok(b'a')))));
        assert!(matches!(iter.next(), Some(((2, 5), Ok(b'r')))));
        assert!(matches!(iter.next(), Some(((2, 6), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((2, 7), Ok(b',')))));
        assert!(matches!(iter.next(), Some(((2, 8), Ok(b'\n')))));

        assert!(matches!(iter.next(), Some(((3, 0), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((3, 1), Ok(b' ')))));
        assert!(matches!(iter.next(), Some(((3, 2), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((3, 3), Ok(b'b')))));
        assert!(matches!(iter.next(), Some(((3, 4), Ok(b'a')))));
        assert!(matches!(iter.next(), Some(((3, 5), Ok(b'z')))));
        assert!(matches!(iter.next(), Some(((3, 6), Ok(b'"')))));
        assert!(matches!(iter.next(), Some(((3, 7), Ok(b'\n')))));

        assert!(matches!(iter.next(), Some(((4, 0), Ok(b']')))));
        assert!(matches!(iter.next(), None));
        assert!(matches!(iter.next(), None));
        assert!(matches!(iter.next(), None));
    }
}
