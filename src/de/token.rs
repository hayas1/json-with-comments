use std::{io, iter::Peekable};

use crate::error::{NeverFail, SyntaxError};

use super::position::{PosRange, Position};

pub struct Tokenizer<R>
where
    R: io::Read,
{
    iter: Peekable<RowColIterator<io::Bytes<R>>>,
}
impl<R> Tokenizer<R>
where
    R: io::Read,
{
    pub fn new(reader: R) -> Self {
        Tokenizer { iter: RowColIterator::new(reader.bytes()).peekable() }
    }
}
impl<R> Tokenizer<R>
where
    R: io::Read,
{
    pub fn eat(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.next() {
            Some((pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(crate::Error::new(e.to_string())), // TODO handling io error
            None => Ok(None),
        }
    }

    pub fn find(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.peek() {
            Some(&(pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(crate::Error::new(e.to_string())), // TODO handling io error
            None => Ok(None),
        }
    }

    pub fn eat_whitespace(&mut self) -> crate::Result<Option<(Position, u8)>> {
        while let Some((pos, c)) = self.eat()? {
            if !c.is_ascii_whitespace() {
                return Ok(Some((pos, c)));
            }
        }
        Ok(None)
    }

    pub fn skip_whitespace(&mut self) -> crate::Result<Option<(Position, u8)>> {
        while let Some((pos, c)) = self.find()? {
            if c.is_ascii_whitespace() {
                self.eat()?;
            } else {
                return Ok(Some((pos, c)));
            }
        }
        Ok(None)
    }

    pub fn parse_string(&mut self) -> crate::Result<Vec<u8>> {
        match self.eat_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingString)? {
            (_, b'"') => {
                let string = self.parse_string_content()?;
                match self.eat()?.ok_or(SyntaxError::EofWhileEndParsingString)? {
                    (_, b'"') => Ok(string),
                    (pos, found) => Err(SyntaxError::UnexpectedTokenWhileEndParsingString { pos, found })?,
                }
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingString { pos, found })?,
        }
    }

    pub fn parse_string_content(&mut self) -> crate::Result<Vec<u8>> {
        let mut buff = Vec::new();
        while let Some((pos, found)) = self.find()? {
            match found {
                b'\\' => buff.push(self.parse_escape_sequence()?),
                b'"' => return Ok(buff),
                c if c.is_ascii_control() => Err(SyntaxError::ControlCharacterWhileParsingString { pos, c })?,
                _ => buff.push(self.eat()?.ok_or(NeverFail::EatAfterFind)?.1),
            }
        }
        Err(SyntaxError::EofWhileEndParsingString)? // TODO contain parsed string?
    }

    pub fn parse_escape_sequence(&mut self) -> crate::Result<u8> {
        unimplemented!("escape sequence")
    }

    pub fn parse_unicode(&mut self) -> crate::Result<u8> {
        unimplemented!("escape unicode")
    }

    pub fn parse_like<F: Fn(u8) -> bool>(&mut self, max: usize, f: F) -> crate::Result<(PosRange, Vec<u8>)> {
        let (start, _) = self.skip_whitespace()?.ok_or(SyntaxError::EofWhileParsingIdent)?;
        let (mut end, mut buff) = (start, Vec::new());
        while let Some((_pos, c)) = self.find()? {
            if f(c) && buff.len() < max {
                (end, _) = self.eat()?.ok_or(NeverFail::EatAfterFind)?;
                buff.push(c)
            } else {
                break;
            }
        }
        Ok(((start, end), buff))
    }

    pub fn parse_ident<T>(&mut self, ident: &[u8], value: T) -> crate::Result<T> {
        let max = 10; // to prevent from parsing tokens that are too long. the longest json ident is `false` of 5.
        let (pos, parsed) = self.parse_like(max, |c| c.is_ascii_alphanumeric() || ident.contains(&c))?;
        if &parsed == ident {
            Ok(value)
        } else {
            Err(SyntaxError::UnexpectedIdent { pos, expected: ident.into(), found: parsed })?
        }
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
        //
        let raw = vec!["[", r#"  "foo","#, r#"  "bar","#, r#"  "baz""#, "]", ""].join("\n");
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
        assert!(matches!(iter.next(), Some(((4, 1), Ok(b'\n')))));

        assert!(matches!(iter.next(), None));
        assert!(matches!(iter.next(), None));
        assert!(matches!(iter.next(), None));
    }

    #[test]
    fn behavior_tokenizer() {
        let raw = r#"
            [
                "jsonc",
                123,
                true,
                false,
                null,
            ]
        "#;
        let reader = BufReader::new(raw.as_bytes());
        let mut tokenizer = Tokenizer::new(reader);

        assert_eq!(tokenizer.find().unwrap(), Some(((0, 0), b'\n')));
        assert_eq!(tokenizer.find().unwrap(), Some(((0, 0), b'\n')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 0), b'\n')));
        assert_eq!(tokenizer.find().unwrap(), Some(((1, 0), b' ')));
        assert_eq!(tokenizer.find().unwrap(), Some(((1, 0), b' ')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((1, 0), b' ')));

        assert_eq!(tokenizer.eat_whitespace().unwrap(), Some(((1, 12), b'[')));
        assert_eq!(tokenizer.find().unwrap(), Some(((1, 13), b'\n')));
        assert_eq!(tokenizer.skip_whitespace().unwrap(), Some(((2, 16), b'"')));
        assert_eq!(tokenizer.find().unwrap(), Some(((2, 16), b'"')));

        assert_eq!(tokenizer.parse_string().unwrap(), b"jsonc");
        assert!(matches!(tokenizer.eat(), Ok(Some((_, b',')))));

        assert!(matches!(tokenizer.skip_whitespace(), Ok(Some((_, b'1')))));
        assert!(matches!(tokenizer.parse_ident(b"123", 123), Ok(123)));
        assert!(matches!(tokenizer.eat(), Ok(Some((_, b',')))));

        assert!(matches!(tokenizer.skip_whitespace(), Ok(Some((_, b't')))));
        assert!(matches!(tokenizer.parse_ident(b"true", true), Ok(true)));
        assert!(matches!(tokenizer.eat(), Ok(Some((_, b',')))));

        assert!(matches!(tokenizer.skip_whitespace(), Ok(Some((_, b'f')))));
        assert!(matches!(tokenizer.parse_ident(b"false", false), Ok(false)));
        assert!(matches!(tokenizer.eat(), Ok(Some((_, b',')))));

        assert!(matches!(tokenizer.skip_whitespace(), Ok(Some((_, b'n')))));
        assert!(matches!(tokenizer.parse_ident(b"null", ()), Ok(())));
        assert!(matches!(tokenizer.eat(), Ok(Some((_, b',')))));

        assert_eq!(tokenizer.eat_whitespace().unwrap(), Some(((7, 12), b']')));
        assert_eq!(tokenizer.find().unwrap(), Some(((7, 13), b'\n')));
        assert_eq!(tokenizer.eat_whitespace().unwrap(), None);
    }

    #[test]
    fn test_parse_string() {
        let parse = |s: &str| Tokenizer::new(s.as_bytes()).parse_string();
        let parse_content = |s: &str| Tokenizer::new(s.as_bytes()).parse_string_content();

        // ok
        assert_eq!(parse(r#""""#).unwrap(), b"");
        assert_eq!(parse(r#""rust""#).unwrap(), b"rust");

        // err
        assert!(matches!(
            parse_content("line\nfeed").unwrap_err().into_inner().downcast_ref().unwrap(),
            SyntaxError::ControlCharacterWhileParsingString { c: b'\n', .. }
        ));
    }
}
