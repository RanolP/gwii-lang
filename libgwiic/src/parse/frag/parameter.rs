use crate::parse::prelude::*;

pub fn parameter<'a>(s: ParserSpan<'a>) -> ParseResult<'a, Parameter> {
    Spanned::wrap(|s| {
        let (s, name) = take_while1(is_text_char)(s)?;

        let s = multispace(s);
        let (s, _) = tag("/")(s)?;

        let s = multispace(s);
        let (s, particle) = take_while1(is_text_char)(s)?;

        return Ok((
            s,
            Parameter {
                name: ParticledName {
                    name: String::from(*name.fragment()),
                    particle: String::from(*particle.fragment()),
                },
            },
        ));
    })(s)
}
