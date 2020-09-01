use crate::parse::prelude::*;

pub fn boolean<'a>(s: ParserSpan<'a>) -> ParseResult<'a, Atom> {
    Spanned::wrap(|s| match opt(tag("참"))(s) {
        (s, Some(_)) => Ok((s, Atom::Boolean(true))),
        (_, None) => tag("거짓")(s).map(|(s, _)| (s, Atom::Boolean(false))),
    })(s)
}
