mod verb;

use crate::parse::prelude::*;

pub fn parse_decl<'a>(s: ParserSpan<'a>) -> ParseResult<'a, Node> {
    // TODO: 당장은 동사 선언 밖에 없어서 ㅋㅋㅋㅋㅋ
    let (s, decl) = alt((verb::parse_verb_decl, verb::parse_verb_decl))(s)?;

    Ok((s, decl.map(|decl| Node::Decl(decl))))
}
