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
    pub fn new(read: R) -> Self {
        ByteTokenizer { iter: RowColIterator::new(read.bytes()).peekable() }
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
    use std::str::FromStr;

    use crate::error::SyntaxError;

    use super::*;

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
