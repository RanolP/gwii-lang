use crate::parse::decl::parse_decl;
use crate::parse::prelude::*;

pub fn parse_gwii<'a>(s: ParserSpan<'a>) -> ParseResultNaked<'a, Vec<Spanned<Node>>> {
    all_consuming(many0(preceded(
        multispace_raw,
        alt((parse_decl, parse_decl)),
    )))(s)
}
