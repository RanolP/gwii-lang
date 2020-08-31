use crate::parse::prelude::*;

pub fn parse_verb_decl<'a>(s: ParserSpan<'a>) -> ParseResult<'a, Decl> {
    Spanned::wrap(|s| {
        let (s, is_extern) = opt(tag("바깥"))(s)?;
        let is_extern = is_extern.is_some();

        let s = multispace(s);
        let (s, _) = tag("동사")(s)?;

        let s = multispace(s);
        let (s, _) = tag(":")(s)?;

        let (s, parameters) = many0(|s| {
            let s = multispace(s);
            let tmp: ParseResultNaked<_> = take(1usize)(s);
            match tmp {
                Ok((_, v)) => {
                    if *v.fragment() == "`" {
                        return Err(NomErr::Error(NomParseError::from_error_kind(
                            s,
                            NomErrorKind::Eof,
                        )));
                    }
                }
                Err(e) => return Err(e),
            }
            parameter(s)
        })(s)?;

        let s = multispace(s);
        let (s, _) = tag("`")(s)?;
        let (s, name) = take_until("`")(s)?;
        let name = String::from(name.fragment().clone());
        let (s, _) = tag("`")(s)?;

        Ok((
            s,
            Decl::Verb(VerbDecl {
                is_extern,
                parameters,
                name,
            }),
        ))
    })(s)
}
