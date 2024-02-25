use std::{io, iter::Peekable};

use crate::de::position::Position;

use super::{RowColIterator, Tokenizer};

pub struct ByteTokenizer<R>
where
    R: io::Read,
{
    iter: Peekable<RowColIterator<io::Bytes<R>>>,
}
impl<R> ByteTokenizer<R>
where
    R: io::Read,
{
    pub fn new(reader: R) -> Self {
        ByteTokenizer { iter: RowColIterator::new(reader.bytes()).peekable() }
    }
}

impl<R> Tokenizer for ByteTokenizer<R>
where
    R: io::Read,
{
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.next() {
            Some((pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(crate::Error::new(e.to_string())), // TODO handling io error
            None => Ok(None),
        }
    }

    fn find(&mut self) -> crate::Result<Option<(Position, u8)>> {
        match self.iter.peek() {
            Some(&(pos, Ok(c))) => Ok(Some((pos, c))),
            Some((_, Err(e))) => Err(crate::Error::new(e.to_string())), // TODO handling io error
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io::BufReader, str::FromStr};

    use crate::error::SyntaxError;

    use super::*;

    #[test]
    fn behavior_fold_token() {
        let raw = r#"[123, 456]"#;
        let reader = BufReader::new(raw.as_bytes());
        let mut tokenizer = ByteTokenizer::new(reader);

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
        let raw = r#"[true, fal, nulled, nul,]"#;
        let reader = BufReader::new(raw.as_bytes());
        let mut tokenizer = ByteTokenizer::new(reader);

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
        let mut tokenizer = ByteTokenizer::new(reader);

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
            ByteTokenizer::new(s.as_bytes()).parse_string().unwrap()
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
            ByteTokenizer::new(s.as_bytes()).parse_string().unwrap_err().into_inner()
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
            ByteTokenizer::new(s.as_bytes()).parse_number().unwrap()
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
            ByteTokenizer::new(s.as_bytes()).parse_number::<T>().unwrap_err().into_inner()
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
