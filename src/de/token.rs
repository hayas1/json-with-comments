pub mod byte;

use std::{io, str::FromStr};

use crate::error::{NeverFail, SyntaxError};

use super::position::{PosRange, Position};

pub trait Tokenizer {
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
    use std::{
        fmt::Debug,
        io::{BufReader, Read},
    };

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

    pub fn behavior_fold_token<'a, T: 'a + Tokenizer, F: Fn(&'a str) -> T>(from: F) {
        let target = r#"[123, 456]"#;
        let mut tokenizer = from(target);

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

    pub fn behavior_parse_ident<'a, T: 'a + Tokenizer, F: Fn(&'a str) -> T>(from: F) {
        let target = r#"[true, fal, nulled, nul,]"#;
        let mut tokenizer = from(target);

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

        assert_eq!(tokenizer.parse_ident(b"null", ()).unwrap(), ());
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 16), b'e')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 17), b'd')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 18), b',')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 19), b' ')));

        match tokenizer.parse_ident(b"null", ()).unwrap_err().into_inner().downcast_ref().unwrap() {
            SyntaxError::UnexpectedIdent { pos, expected, found } => {
                assert_eq!(pos, &((0, 20), (0, 22)));
                assert_eq!(expected, &b"null".to_vec());
                assert_eq!(found, &b"nul".to_vec());
            }
            _ => unreachable!(),
        }
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 23), b',')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 24), b']')));

        assert!(matches!(
            tokenizer.parse_ident(b"None", ()).unwrap_err().into_inner().downcast_ref().unwrap(),
            SyntaxError::EofWhileParsingIdent,
        ));
        assert_eq!(tokenizer.parse_ident(b"", ()).unwrap(), ());
    }

    pub fn behavior_tokenizer<'a, T: 'a + Tokenizer, F: Fn(&'a str) -> T>(from: F) {
        let target = r#"
            [
                "jsonc",
                123,
                true,
                false,
                null,
            ]
        "#;
        let mut tokenizer = from(target);

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

    pub fn behavior_parse_owned_string<'a, T: 'a + Tokenizer, F: Fn(&'a str) -> T>(from: F) {
        fn parse(mut tokenizer: impl Tokenizer) -> Vec<u8> {
            tokenizer.parse_string().unwrap()
        }

        assert_eq!(parse(from(r#""""#)), b"");
        assert_eq!(parse(from(r#""rust""#)), b"rust");
        assert_eq!(parse(from(r#""\"quote\"""#)), b"\"quote\"");
        assert_eq!(parse(from(r#""back\\slash""#)), b"back\\slash");
        assert_eq!(parse(from(r#""escaped\/slash""#)), b"escaped/slash");
        assert_eq!(parse(from(r#""unescaped/slash""#)), b"unescaped/slash");
        assert_eq!(parse(from(r#""backspace\b formfeed\f""#)), b"backspace\x08 formfeed\x0C");
        assert_eq!(parse(from(r#""line\nfeed""#)), b"line\nfeed");
        assert_eq!(parse(from(r#""white\tspace""#)), b"white\tspace");
        assert_eq!(String::from_utf8(parse(from(r#""line\u000Afeed""#))).unwrap(), "line\u{000A}feed");
        assert_eq!(parse(from(r#""line\u000Afeed""#)), "line\nfeed".bytes().collect::<Vec<_>>());
        assert_eq!(parse(from(r#""epsilon \u03b5""#)), "epsilon ε".bytes().collect::<Vec<_>>());
        assert_eq!(parse(from(r#""💯""#)), "💯".bytes().collect::<Vec<_>>());
    }

    pub fn behavior_parse_owned_string_err<'a, T: 'a + Tokenizer, F: Fn(&'a str) -> T>(from: F) {
        fn parse_err(mut tokenizer: impl Tokenizer) -> Box<dyn std::error::Error + Send + Sync> {
            tokenizer.parse_string().unwrap_err().into_inner()
        }

        assert!(matches!(
            parse_err(from(r#""ending..."#)).downcast_ref().unwrap(),
            SyntaxError::EofWhileEndParsingString,
        ));
        assert!(matches!(
            parse_err(from(
                r#""line
                    feed""#
            ))
            .downcast_ref()
            .unwrap(),
            SyntaxError::ControlCharacterWhileParsingString { c: b'\n', .. }
        ));
        assert!(matches!(
            parse_err(from(r#""escape EoF \"#)).downcast_ref().unwrap(),
            SyntaxError::EofWhileParsingEscapeSequence,
        ));
        assert!(matches!(
            parse_err(from(r#""invalid escape sequence \a""#)).downcast_ref().unwrap(),
            SyntaxError::InvalidEscapeSequence { found: b'a', .. }
        ));
        assert!(matches!(
            parse_err(from(r#""invalid unicode \uXXXX""#)).downcast_ref().unwrap(),
            SyntaxError::InvalidUnicodeEscape { found: b'X', .. }
        ))
    }

    pub fn behavior_parse_number<'a, T: 'a + Tokenizer, F: Fn(&'a str) -> T>(from: F) {
        fn parse<U: FromStr>(mut tokenizer: impl Tokenizer) -> U {
            tokenizer.parse_number().unwrap()
        }

        assert_eq!(parse::<u8>(from("255")), 255);
        assert_eq!(parse::<u16>(from("16")), 16);
        assert_eq!(parse::<u32>(from("32")), 32);
        assert_eq!(parse::<u64>(from("9999999999999999999")), 9999999999999999999);
        assert_eq!(
            parse::<u128>(from("340282366920938463463374607431768211455")),
            340282366920938463463374607431768211455
        );
        assert_eq!(parse::<i8>(from("-127")), -127);
        assert_eq!(parse::<i16>(from("16")), 16);
        assert_eq!(parse::<i32>(from("-32")), -32);
        assert_eq!(parse::<i64>(from("-999999999999999999")), -999999999999999999);
        assert_eq!(
            parse::<i128>(from("-170141183460469231731687303715884105728")),
            -170141183460469231731687303715884105728
        );
        assert_eq!(parse::<f32>(from("0.000000000000000e00000000000000000")), 0.);
        assert_eq!(parse::<f32>(from("3.1415926535")), 3.1415926535);
        assert_eq!(parse::<f32>(from("2.7")), 2.7);
        assert_eq!(parse::<f64>(from("8.314462618")), 8.314462618);
        assert_eq!(parse::<f64>(from("6.674e-11")), 0.00000000006674);
        assert_eq!(parse::<f64>(from("6.02214076e23")), 6.02214076E23);
    }

    pub fn behavior_parse_number_err<'a, T: 'a + Tokenizer, F: Fn(&'a str) -> T>(from: F) {
        fn parse_err<U: FromStr + Debug>(mut tokenizer: impl Tokenizer) -> Box<dyn std::error::Error + Send + Sync> {
            tokenizer.parse_number::<U>().unwrap_err().into_inner()
        }

        assert!(matches!(parse_err::<u8>(from("256")).downcast_ref().unwrap(), SyntaxError::InvalidNumber { .. }));
        assert!(matches!(
            parse_err::<u32>(from("000")).downcast_ref().unwrap(),
            SyntaxError::InvalidLeadingZeros { .. }
        ));
        assert!(matches!(
            parse_err::<u32>(from("012")).downcast_ref().unwrap(),
            SyntaxError::InvalidLeadingZeros { .. }
        ));
        assert!(matches!(
            parse_err::<u32>(from("+12")).downcast_ref().unwrap(),
            SyntaxError::InvalidLeadingPlus { .. }
        ));
        assert!(matches!(
            parse_err::<i32>(from("-999999999999")).downcast_ref().unwrap(),
            SyntaxError::InvalidNumber { .. }
        ));
        assert!(matches!(
            parse_err::<f32>(from("0.")).downcast_ref().unwrap(),
            SyntaxError::EofWhileStartParsingFraction { .. },
        ));
        assert!(matches!(
            parse_err::<f32>(from("0.e")).downcast_ref().unwrap(),
            SyntaxError::MissingFraction { found: b'e', .. },
        ));
        assert!(matches!(
            parse_err::<f64>(from("0e")).downcast_ref().unwrap(),
            SyntaxError::EofWhileStartParsingExponent { .. },
        ));
        assert!(matches!(
            parse_err::<f64>(from("1e.")).downcast_ref().unwrap(),
            SyntaxError::MissingExponent { found: b'.', .. },
        ));
    }
}
