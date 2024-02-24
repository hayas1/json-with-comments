use std::{io, iter::Peekable, str::FromStr};

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

    pub fn fold_token<F: FnMut(&[u8], u8) -> bool>(&mut self, mut f: F) -> crate::Result<(Option<PosRange>, Vec<u8>)> {
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

    pub fn parse_ident<T>(&mut self, ident: &[u8], value: T) -> crate::Result<T> {
        let (mut iter, mut ok) = (ident.into_iter(), true);
        let (p, parsed) = self.fold_token(|_, c| {
            if c.is_ascii_alphanumeric() || matches!(c, b'_') {
                ok &= iter.next().map_or(false, |&i| i == c);
                true // keep parsing ident, even if does not match ident and parsed
            } else {
                false
            }
        })?;
        match (p, ok && iter.next().is_none()) {
            (_, true) => Ok(value),
            (Some(pos), false) => Err(SyntaxError::UnexpectedIdent { pos, expected: ident.into(), found: parsed })?,
            (None, false) => Err(SyntaxError::EofWhileParsingIdent)?,
        }
    }

    pub fn parse_string(&mut self) -> crate::Result<Vec<u8>> {
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

    pub fn parse_string_content(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
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

    pub fn parse_escape_sequence(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
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

    pub fn parse_unicode(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
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

    pub fn parse_number<T: FromStr>(&mut self) -> crate::Result<T> {
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

    #[test]
    fn behavior_fold_token() {
        let raw = r#"[123, 456]"#;
        let reader = BufReader::new(raw.as_bytes());
        let mut tokenizer = Tokenizer::new(reader);

        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 0), b'[')));
        assert_eq!(
            tokenizer.fold_token(|_t, c| matches!(c, b'1'..=b'9')).unwrap(),
            (Some(((0, 1), (0, 3))), vec![b'1', b'2', b'3']),
        );
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 4), b',')));
        assert_eq!(tokenizer.fold_token(|_t, c| matches!(c, b'1'..=b'9')).unwrap(), (Some(((0, 5), (0, 5))), vec![]));
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 5), b' ')));
        assert_eq!(
            tokenizer.fold_token(|_t, c| matches!(c, b'1'..=b'9')).unwrap(),
            (Some(((0, 6), (0, 8))), vec![b'4', b'5', b'6']),
        );
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 9), b']')));
        assert_eq!(tokenizer.eat().unwrap(), None);
        assert_eq!(tokenizer.fold_token(|_t, c| matches!(c, b'1'..=b'9')).unwrap(), (None, vec![]));
        assert_eq!(tokenizer.eat().unwrap(), None);
    }

    #[test]
    fn behavior_parse_ident() {
        let raw = r#"[true, fal, nulling]"#;
        let reader = BufReader::new(raw.as_bytes());
        let mut tokenizer = Tokenizer::new(reader);

        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 0), b'[')));
        assert_eq!(tokenizer.parse_ident(b"true", true).unwrap(), true);
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 5), b',')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 6), b' ')));

        match tokenizer.parse_ident(b"false", false).unwrap_err().into_inner().downcast_ref().unwrap() {
            SyntaxError::UnexpectedIdent { pos, expected, found } => {
                assert_eq!(pos, &((0, 7), (0, 9)));
                assert_eq!(expected, &b"false".to_vec());
                assert_eq!(found, &b"fal".to_vec());
            }
            _ => unreachable!(),
        }
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 10), b',')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 11), b' ')));

        match tokenizer.parse_ident(b"null", ()).unwrap_err().into_inner().downcast_ref().unwrap() {
            SyntaxError::UnexpectedIdent { pos, expected, found } => {
                assert_eq!(pos, &((0, 12), (0, 18)));
                assert_eq!(expected, &b"null".to_vec());
                assert_eq!(found, &b"nulling".to_vec());
            }
            _ => unreachable!(),
        }
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 19), b']')));

        assert!(matches!(
            tokenizer.parse_ident(b"None", ()).unwrap_err().into_inner().downcast_ref().unwrap(),
            SyntaxError::EofWhileParsingIdent,
        ));
        assert_eq!(tokenizer.parse_ident(b"", ()).unwrap(), ());
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
        assert!(matches!(tokenizer.parse_number(), Ok(123)));
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
        // ok
        fn parse(s: &str) -> Vec<u8> {
            Tokenizer::new(s.as_bytes()).parse_string().unwrap()
        }
        assert_eq!(parse(r#""""#), b"");
        assert_eq!(parse(r#""rust""#), b"rust");
        assert_eq!(parse(r#""\"quote\"""#), b"\"quote\"");
        assert_eq!(parse(r#""back\\slash""#), b"back\\slash");
        assert_eq!(parse(r#""escaped\/slash""#), b"escaped/slash");
        assert_eq!(parse(r#""unescaped/slash""#), b"unescaped/slash");
        assert_eq!(parse(r#""backspace\b formfeed\f""#), b"backspace\x08 formfeed\x0C");
        assert_eq!(parse(r#""line\nfeed""#), b"line\nfeed");
        assert_eq!(parse(r#""white\tspace""#), b"white\tspace");
        assert_eq!(String::from_utf8(parse(r#""line\u000Afeed""#)).unwrap(), "line\u{000A}feed");
        assert_eq!(parse(r#""line\u000Afeed""#), "line\nfeed".bytes().collect::<Vec<_>>());
        assert_eq!(parse(r#""epsilon \u03b5""#), "epsilon ε".bytes().collect::<Vec<_>>());
        assert_eq!(parse(r#""💯""#), "💯".bytes().collect::<Vec<_>>());

        // err
        fn parse_err(s: &str) -> Box<dyn std::error::Error> {
            Tokenizer::new(s.as_bytes()).parse_string().unwrap_err().into_inner()
        }
        assert!(matches!(parse_err(r#""ending..."#).downcast_ref().unwrap(), SyntaxError::EofWhileEndParsingString,));
        assert!(matches!(
            parse_err(
                r#""line
                    feed""#
            )
            .downcast_ref()
            .unwrap(),
            SyntaxError::ControlCharacterWhileParsingString { c: b'\n', .. }
        ));
        assert!(matches!(
            parse_err(r#""escape EoF \"#).downcast_ref().unwrap(),
            SyntaxError::EofWhileParsingEscapeSequence,
        ));
        assert!(matches!(
            parse_err(r#""invalid escape sequence \a""#).downcast_ref().unwrap(),
            SyntaxError::InvalidEscapeSequence { found: b'a', .. }
        ));
        assert!(matches!(
            parse_err(r#""invalid unicode \uXXXX""#).downcast_ref().unwrap(),
            SyntaxError::InvalidUnicodeEscape { found: b'X', .. }
        ))
    }

    #[test]
    fn test_parse_number() {
        // ok
        fn parse<T: FromStr>(s: &str) -> T {
            Tokenizer::new(s.as_bytes()).parse_number().unwrap()
        }
        assert_eq!(parse::<u8>("255"), 255);
        assert_eq!(parse::<u16>("16"), 16);
        assert_eq!(parse::<u32>("32"), 32);
        assert_eq!(parse::<u64>("9999999999999999999"), 9999999999999999999);
        assert_eq!(parse::<u128>("340282366920938463463374607431768211455"), 340282366920938463463374607431768211455);
        assert_eq!(parse::<i8>("-127"), -127);
        assert_eq!(parse::<i16>("16"), 16);
        assert_eq!(parse::<i32>("-32"), -32);
        assert_eq!(parse::<i64>("-999999999999999999"), -999999999999999999);
        assert_eq!(parse::<i128>("-170141183460469231731687303715884105728"), -170141183460469231731687303715884105728);
        assert_eq!(parse::<f32>("0.000000000000000e00000000000000000"), 0.);
        assert_eq!(parse::<f32>("3.1415926535"), 3.1415926535);
        assert_eq!(parse::<f32>("2.7"), 2.7);
        assert_eq!(parse::<f64>("8.314462618"), 8.314462618);
        assert_eq!(parse::<f64>("6.674e-11"), 0.00000000006674);
        assert_eq!(parse::<f64>("6.02214076e23"), 6.02214076E23);

        // err
        fn parse_err<T: FromStr + std::fmt::Debug>(s: &str) -> Box<dyn std::error::Error + Send + Sync> {
            Tokenizer::new(s.as_bytes()).parse_number::<T>().unwrap_err().into_inner()
        }
        assert!(matches!(parse_err::<u32>("000").downcast_ref().unwrap(), SyntaxError::InvalidLeadingZeros { .. }));
        assert!(matches!(parse_err::<u32>("012").downcast_ref().unwrap(), SyntaxError::InvalidLeadingZeros { .. }));
        assert!(matches!(parse_err::<u32>("+12").downcast_ref().unwrap(), SyntaxError::InvalidLeadingPlus { .. }));
        assert!(matches!(parse_err::<u8>("256").downcast_ref().unwrap(), SyntaxError::InvalidNumber { .. }));
        assert!(matches!(parse_err::<i32>("-999999999999").downcast_ref().unwrap(), SyntaxError::InvalidNumber { .. }));
        assert!(matches!(
            parse_err::<f32>("0.").downcast_ref().unwrap(),
            SyntaxError::EofWhileStartParsingFraction { .. },
        ));
        assert!(matches!(
            parse_err::<f32>("0.e").downcast_ref().unwrap(),
            SyntaxError::MissingFraction { found: b'e', .. },
        ));
        assert!(matches!(
            parse_err::<f64>("0e").downcast_ref().unwrap(),
            SyntaxError::EofWhileStartParsingExponent { .. },
        ));
        assert!(matches!(
            parse_err::<f64>("1e.").downcast_ref().unwrap(),
            SyntaxError::MissingExponent { found: b'.', .. },
        ));
    }
}
