use crate::parse::prelude::*;

pub fn integer<'a>(s: ParserSpan<'a>) -> ParseResult<'a, Atom> {
    alt((integer_decimal, integer_binary, integer_octal, integer_hex))(s)
}

pub fn integer_decimal<'a>(s: ParserSpan<'a>) -> ParseResult<'a, Atom> {
    Spanned::wrap(|s| {
        let (s, head) = one_of("123456789")(s)?;
        let (s, tail) = take_while1(|c| matches!(c, '0'..='7'))(s)?;

        Ok((
            s,
            Atom::Integer(IntegerLiteralKind::Decimal, {
                let mut str = String::new();
                str.push(head);
                str.push_str(*tail.fragment());
                str
            }),
        ))
    })(s)
}

pub fn integer_binary<'a>(s: ParserSpan<'a>) -> ParseResult<'a, Atom> {
    Spanned::wrap(|s| {
        let (s, _) = alt((tag("02:"), tag("0b")))(s)?;
        let (s, num) = take_while1(|c| matches!(c, '0' | '1'))(s)?;

        Ok((
            s,
            Atom::Integer(IntegerLiteralKind::Binary, String::from(*num.fragment())),
        ))
    })(s)
}

pub fn integer_octal<'a>(s: ParserSpan<'a>) -> ParseResult<'a, Atom> {
    Spanned::wrap(|s| {
        let (s, _) = alt((tag("08:"), tag("0o")))(s)?;
        let (s, num) = take_while1(|c| matches!(c, '0'..='7'))(s)?;

        Ok((
            s,
            Atom::Integer(IntegerLiteralKind::Octal, String::from(*num.fragment())),
        ))
    })(s)
}

pub fn integer_hex<'a>(s: ParserSpan<'a>) -> ParseResult<'a, Atom> {
    Spanned::wrap(|s| {
        let (s, _) = alt((tag("08:"), tag("0o")))(s)?;
        let (s, num) =
            take_while1(|c| matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F' | '가'..='바'))(s)?;

        Ok((
            s,
            Atom::Integer(IntegerLiteralKind::Hex, String::from(*num.fragment())),
        ))
    })(s)
}
