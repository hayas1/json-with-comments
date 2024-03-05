pub mod raw;
pub mod read;
pub mod slice;
pub mod str;

use crate::{
    error::{Ensure, SemanticError, SyntaxError},
    value::string::StringValue,
};

use super::position::{PosRange, Position};

pub trait Tokenizer<'de> {
    fn eat(&mut self) -> crate::Result<Option<(Position, u8)>>;
    fn look(&mut self) -> crate::Result<Option<(Position, u8)>>;

    fn eat_whitespace(&mut self) -> crate::Result<Option<(Position, u8)>> {
        loop {
            match self.eat()? {
                Some((_, b'/')) => _ = self.eat_comment_follow()?,
                Some((_, c)) if c.is_ascii_whitespace() => (),
                Some((pos, c)) => return Ok(Some((pos, c))),
                None => return Ok(None),
            }
        }
    }

    fn skip_whitespace(&mut self) -> crate::Result<Option<(Position, u8)>> {
        loop {
            match self.look()? {
                Some((_, b'/')) => _ = self.eat_comment()?,
                Some((_, c)) if c.is_ascii_whitespace() => _ = self.eat()?,
                Some((pos, c)) => return Ok(Some((pos, c))),
                None => return Ok(None),
            }
        }
    }

    fn eat_comment(&mut self) -> crate::Result<Option<(PosRange, Vec<u8>)>> {
        match self.eat()?.ok_or(SyntaxError::EofWhileStartParsingComment)? {
            (start, b'/') => Ok(self.eat_comment_follow()?.map(|(e, c)| ((start, e), c))),
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingComment { pos, found })?,
        }
    }

    fn eat_comment_follow(&mut self) -> crate::Result<Option<(Position, Vec<u8>)>> {
        match self.eat()?.ok_or(SyntaxError::EofWhileStartParsingComment)? {
            (follow, b'/') => {
                let mut content = b"//".to_vec();
                let end = self.eat_slash_comment_content(&mut content)?;
                Ok(Some(((end.unwrap_or(follow)), content.to_vec())))
            }
            (follow, b'*') => {
                let mut content = b"/*".to_vec();
                let end = self.eat_asterisk_comment_content(&mut content)?;
                content.extend_from_slice(b"*/");
                Ok(Some(((end.unwrap_or(follow)), content.to_vec())))
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingComment { pos, found })?,
        }
    }

    fn eat_slash_comment_content(&mut self, buff: &mut Vec<u8>) -> crate::Result<Option<Position>> {
        while let Some((pos, c)) = self.eat()? {
            match c {
                b'\n' => return Ok(Some(pos)),
                _ => buff.push(c),
            }
        }
        Ok(None)
    }

    fn eat_asterisk_comment_content(&mut self, buff: &mut Vec<u8>) -> crate::Result<Option<Position>> {
        while let Some((_, c)) = self.eat()? {
            match c {
                b'*' => match self.look()?.ok_or(SyntaxError::UnterminatedComment)? {
                    (_, b'/') => return Ok(Some(self.eat()?.ok_or(Ensure::EatAfterLook)?.0)),
                    _ => buff.push(c),
                },
                _ => buff.push(c),
            }
        }
        Err(SyntaxError::UnterminatedComment)?
    }

    fn fold_token<F: FnMut(&[u8], u8) -> bool>(&mut self, mut f: F) -> crate::Result<(Option<PosRange>, Vec<u8>)> {
        let (mut range, mut buff) = (None, Vec::new());
        while let Some((pos, c)) = self.look()? {
            range = if range.is_none() { Some((pos, pos)) } else { range };
            if f(&buff, c) {
                let (p, c) = self.eat()?.ok_or(Ensure::EatAfterLook)?;
                if let Some((s, _)) = range {
                    range = Some((s, p));
                }
                buff.push(c);
            } else {
                break;
            }
        }
        Ok((range, buff))
    }

    fn parse_ident<T>(&mut self, ident: &[u8], value: T) -> crate::Result<T> {
        let mut iter = ident.iter();
        let (p, parsed) = self.fold_token(|_, c| iter.next().map_or(false, |&i| i == c))?;
        match (p, iter.next().is_none() && parsed.len() == ident.len()) {
            (_, true) => Ok(value),
            (Some(pos), false) => Err(SyntaxError::UnexpectedIdent { pos, expected: ident.into(), found: parsed })?,
            (None, false) => Err(SyntaxError::EofWhileParsingIdent)?,
        }
    }

    fn parse_string(&mut self) -> crate::Result<StringValue<'de>> {
        match self.eat_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingString)? {
            (_, b'"') => {
                let value = self.parse_string_content()?;
                match self.eat()?.ok_or(SyntaxError::EofWhileEndParsingString)? {
                    (_, b'"') => Ok(value),
                    (pos, found) => Err(SyntaxError::UnexpectedTokenWhileEndParsingString { pos, found })?,
                }
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingString { pos, found })?,
        }
    }

    fn parse_string_content(&mut self) -> crate::Result<StringValue<'de>> {
        self.parse_string_content_super()
    }
    fn parse_string_content_super(&mut self) -> crate::Result<StringValue<'de>> {
        let mut buff = Vec::new();
        while let Some((pos, found)) = self.look()? {
            match found {
                b'\\' => self.parse_escape_sequence(&mut buff)?,
                b'"' => return Ok(StringValue::Owned(String::from_utf8(buff)?)),
                c if c.is_ascii_control() => Err(SyntaxError::ControlCharacterWhileParsingString { pos, c })?,
                _ => buff.push(self.eat()?.ok_or(Ensure::EatAfterLook)?.1),
            }
        }
        Err(SyntaxError::EofWhileEndParsingString)? // TODO contain parsed string?
    }

    fn parse_escape_sequence(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
        self.parse_escape_sequence_super(buff)
    }
    fn parse_escape_sequence_super(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
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
                (_, c @ b'0'..=b'9') => hex += ((c - b'0') as u32) << (4 * (3 - i)),
                (_, c @ b'a'..=b'f') => hex += ((c - b'a' + 10) as u32) << (4 * (3 - i)),
                (_, c @ b'A'..=b'F') => hex += ((c - b'A' + 10) as u32) << (4 * (3 - i)),
                (pos, found) => return Err(SyntaxError::InvalidUnicodeEscape { pos, found })?,
            }
        }
        let ch = unsafe { char::from_u32_unchecked(hex) }; // TODO maybe safe
        Ok(buff.extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes()))
    }

    fn parse_number<T: std::str::FromStr>(&mut self) -> crate::Result<T> {
        self.parse_number_super()
    }
    fn parse_number_super<T: std::str::FromStr>(&mut self) -> crate::Result<T> {
        let mut buff = Vec::new();
        let (pos, _) = self.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingNumber)?;
        self.parse_integer_part(&mut buff)?;
        if let Some((_, b'.')) = self.look()? {
            buff.push(self.eat()?.ok_or(Ensure::EatAfterLook)?.1);
            self.parse_fraction_part(&mut buff)?;
        }
        if let Some((_, b'e' | b'E')) = self.look()? {
            buff.push(self.eat()?.ok_or(Ensure::EatAfterLook)?.1);
            self.parse_exponent_part(&mut buff)?;
        }
        let representation = String::from_utf8(buff)?;
        Ok(representation.parse().or(Err(SemanticError::InvalidNumber { pos, rep: representation }))?)
    }

    fn parse_integer_part(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
        match self.skip_whitespace()?.ok_or(SyntaxError::EofWhileStartParsingNumber)? {
            (_, b'-') => buff.push(self.eat()?.ok_or(Ensure::EatAfterLook)?.1),
            (pos, b'+') => Err(SyntaxError::InvalidLeadingPlus { pos })?,
            _ => (),
        }
        match self.eat()?.ok_or(SyntaxError::EofWhileParsingNumber)? {
            (_, c @ b'0') => match self.look()? {
                Some((pos, b'0'..=b'9')) => Err(SyntaxError::InvalidLeadingZeros { pos })?,
                _ => Ok(buff.push(c)),
            },
            (_, c @ b'1'..=b'9') => {
                buff.push(c);
                let (_, remain) = self.fold_token(|_, c| matches!(c, b'0'..=b'9'))?;
                Ok(buff.extend_from_slice(&remain))
            }
            (pos, found) => Err(SyntaxError::UnexpectedTokenWhileStartParsingNumber { pos, found })?,
        }
    }

    fn parse_fraction_part(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
        match self.look()?.ok_or(SyntaxError::EofWhileStartParsingFraction)? {
            (_, b'0'..=b'9') => {
                let (_, fraction) = self.fold_token(|_, c| matches!(c, b'0'..=b'9'))?;
                Ok(buff.extend_from_slice(&fraction))
            }
            (pos, found) => Err(SyntaxError::MissingFraction { pos, found })?,
        }
    }

    fn parse_exponent_part(&mut self, buff: &mut Vec<u8>) -> crate::Result<()> {
        match self.eat()?.ok_or(SyntaxError::EofWhileStartParsingExponent)? {
            (_, c @ (b'+' | b'-' | b'0'..=b'9')) => buff.push(c),
            (pos, found) => Err(SyntaxError::MissingExponent { pos, found })?,
        }
        let (_, exponent) = self.fold_token(|_, c| matches!(c, b'0'..=b'9'))?;
        Ok(buff.extend_from_slice(&exponent))
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, str::FromStr};

    use super::*;

    pub fn behavior_fold_token<'a, T: 'a + Tokenizer<'a>, F: Fn(&'a str) -> T>(from: F) {
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

    pub fn behavior_parse_ident<'a, T: 'a + Tokenizer<'a>, F: Fn(&'a str) -> T>(from: F) {
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

    pub fn behavior_tokenizer<'a, T: 'a + Tokenizer<'a>, F: Fn(&'a str) -> T>(from: F) {
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

        assert_eq!(tokenizer.look().unwrap(), Some(((0, 0), b'\n')));
        assert_eq!(tokenizer.look().unwrap(), Some(((0, 0), b'\n')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((0, 0), b'\n')));
        assert_eq!(tokenizer.look().unwrap(), Some(((1, 0), b' ')));
        assert_eq!(tokenizer.look().unwrap(), Some(((1, 0), b' ')));
        assert_eq!(tokenizer.eat().unwrap(), Some(((1, 0), b' ')));

        assert_eq!(tokenizer.eat_whitespace().unwrap(), Some(((1, 12), b'[')));
        assert_eq!(tokenizer.look().unwrap(), Some(((1, 13), b'\n')));
        assert_eq!(tokenizer.skip_whitespace().unwrap(), Some(((2, 16), b'"')));
        assert_eq!(tokenizer.look().unwrap(), Some(((2, 16), b'"')));

        assert_eq!(tokenizer.parse_string().unwrap(), "jsonc");
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
        assert_eq!(tokenizer.look().unwrap(), Some(((7, 13), b'\n')));
        assert_eq!(tokenizer.eat_whitespace().unwrap(), None);
    }

    pub fn behavior_parse_unescaped_string<'a, T: 'a + Tokenizer<'a>, F: Fn(&'a str) -> T>(from: F) {
        fn parse<'a>(mut tokenizer: impl Tokenizer<'a>) -> String {
            tokenizer.parse_string().unwrap().to_string()
        }

        assert_eq!(parse(from(r#""""#)), "");
        assert_eq!(parse(from(r#""rust""#)), "rust");
        assert_eq!(parse(from(r#""\"quote\"""#)), "\"quote\"");
        assert_eq!(parse(from(r#""back\\slash""#)), "back\\slash");
        assert_eq!(parse(from(r#""escaped\/slash""#)), "escaped/slash");
        assert_eq!(parse(from(r#""unescaped/slash""#)), "unescaped/slash");
        assert_eq!(parse(from(r#""backspace\b formfeed\f""#)), "backspace\x08 formfeed\x0C");
        assert_eq!(parse(from(r#""line\nfeed""#)), "line\nfeed");
        assert_eq!(parse(from(r#""white\tspace""#)), "white\tspace");
        assert_eq!(parse(from(r#""line\u000Afeed""#)), "line\u{000A}feed");
        assert_eq!(parse(from(r#""line\u000Afeed""#)), "line\nfeed");
        assert_eq!(parse(from(r#""epsilon \u03b5""#)), "epsilon ε");
        assert_eq!(parse(from(r#""💯""#)), "💯");
    }

    pub fn behavior_parse_raw_string<'a, T: 'a + Tokenizer<'a>, F: Fn(&'a str) -> T>(from: F) {
        fn parse<'a>(mut tokenizer: impl Tokenizer<'a>) -> &'a str {
            match tokenizer.parse_string().unwrap() {
                StringValue::Borrowed(s) => s,
                StringValue::Owned(s) => panic!("expected borrowed string, got owned: {}", s),
            }
        }

        assert_eq!(parse(from(r#""""#)), "");
        assert_eq!(parse(from(r#""rust""#)), "rust");
        assert_eq!(parse(from(r#""\"quote\"""#)), "\\\"quote\\\"");
        assert_eq!(parse(from(r#""back\\slash""#)), "back\\\\slash");
        assert_eq!(parse(from(r#""escaped\/slash""#)), "escaped\\/slash");
        assert_eq!(parse(from(r#""unescaped/slash""#)), "unescaped/slash");
        assert_eq!(parse(from(r#""backspace\b formfeed\f""#)), "backspace\\b formfeed\\f");
        assert_eq!(parse(from(r#""line\nfeed""#)), "line\\nfeed");
        assert_eq!(parse(from(r#""white\tspace""#)), "white\\tspace");
        assert_eq!(parse(from(r#""line\u000Afeed""#)), "line\\u000Afeed");
        assert_eq!(parse(from(r#""epsilon \u03b5""#)), "epsilon \\u03b5");
        assert_eq!(parse(from(r#""💯""#)), "💯");
    }

    pub fn behavior_parse_string_err<'a, T: 'a + Tokenizer<'a>, F: Fn(&'a str) -> T>(from: F) {
        fn parse_err<'a>(mut tokenizer: impl Tokenizer<'a>) -> Box<dyn std::error::Error + Send + Sync> {
            tokenizer.parse_string().unwrap_err().into_inner()
        }

        assert!(matches!(
            parse_err(from(r#""ending..."#)).downcast_ref().unwrap(),
            SyntaxError::EofWhileEndParsingString,
        ));
        assert!(matches!(
            parse_err(from("\"escape \x1b\"")).downcast_ref().unwrap(),
            SyntaxError::ControlCharacterWhileParsingString { c: b'\x1b', .. },
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

    pub fn behavior_parse_number<'a, T: 'a + Tokenizer<'a>, F: Fn(&'a str) -> T>(from: F) {
        fn parse<'a, U: FromStr>(mut tokenizer: impl Tokenizer<'a>) -> U {
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

    pub fn behavior_parse_number_err<'a, T: 'a + Tokenizer<'a>, F: Fn(&'a str) -> T>(from: F) {
        fn parse_err<'a, U: FromStr + Debug>(
            mut tokenizer: impl Tokenizer<'a>,
        ) -> Box<dyn std::error::Error + Send + Sync> {
            tokenizer.parse_number::<U>().unwrap_err().into_inner()
        }

        assert!(matches!(parse_err::<u8>(from("256")).downcast_ref().unwrap(), SemanticError::InvalidNumber { .. }));
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
            SemanticError::InvalidNumber { .. }
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
