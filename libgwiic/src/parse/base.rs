use crate::parse::{ParseResultNaked, ParserSpan};
use nom::bytes::complete::tag as nom_tag;
use nom::bytes::complete::take_while;
use nom::{Compare, InputLength};

pub(crate) fn tag<'a, T>(
    input: T,
) -> impl Fn(ParserSpan<'a>) -> ParseResultNaked<'a, ParserSpan<'a>>
where
    T: InputLength + Clone,
    ParserSpan<'a>: Compare<T>,
{
    nom_tag(input)
}

pub(crate) fn opt<'a, F, R>(f: F) -> impl Fn(ParserSpan<'a>) -> ParseResultNaked<'a, Option<R>>
where
    F: Fn(ParserSpan<'a>) -> ParseResultNaked<'a, R>,
{
    move |s| match f(s) {
        Ok((s, r)) => Ok((s, Some(r))),
        Err(_) => Ok((s, None)),
    }
}

pub(crate) fn opt_mut<'a, F, R>(
    mut f: F,
) -> impl FnMut(ParserSpan<'a>) -> ParseResultNaked<'a, Option<R>>
where
    F: FnMut(ParserSpan<'a>) -> ParseResultNaked<'a, R>,
{
    move |s| match f(s) {
        Ok((s, r)) => Ok((s, Some(r))),
        Err(_) => Ok((s, None)),
    }
}

pub(crate) fn opt_once<'a, F, R>(
    mut f: F,
) -> impl FnOnce(ParserSpan<'a>) -> ParseResultNaked<'a, Option<R>>
where
    F: FnOnce(ParserSpan<'a>) -> ParseResultNaked<'a, R>,
{
    move |s| match f(s) {
        Ok((s, r)) => Ok((s, Some(r))),
        Err(_) => Ok((s, None)),
    }
}

pub(crate) fn multispace(s: ParserSpan<'_>) -> ParserSpan<'_> {
    multispace_raw(s).unwrap().0
}

pub(crate) fn multispace_raw<'a>(s: ParserSpan<'a>) -> ParseResultNaked<'a, ()> {
    let (s, _) = take_while(is_multispace)(s)?;
    Ok((s, ()))
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CodeCharacterKind {
    HorizontalSpace,
    VerticalSpace,
    Text,
    Punctuation,
}

pub(crate) fn classifier_code_character(c: char) -> CodeCharacterKind {
    match c {
        | '\t'       // CHARACTER TABULATION
        | ' '        // SPACE
        | '\u{00AD}' // SOFT HYPHEN
        | '\u{00A0}' // NO-BREAK SPACE
        | '\u{1680}' // OGHAM SPACE MARK
        | '\u{2000}' // EN QUAD
        | '\u{2001}' // EM QUAD
        | '\u{2002}' // EN SPACE
        | '\u{2003}' // EM SPACE
        | '\u{2004}' // THREE-PER-EM SPACE
        | '\u{2005}' // FOUR-PER-EM SPACE
        | '\u{2006}' // SIX-PER-EM SPACE
        | '\u{2007}' // FIGURE SPACE
        | '\u{2008}' // PUNCTUATION SPACE
        | '\u{2009}' // THIN SPACE
        | '\u{200A}' // HAIR SPACE
        | '\u{200B}' // ZERO WIDTH SPACE
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK
        | '\u{202F}' // NARROW NO-BREAK SPACE
        | '\u{205F}' // MEDIUM MATHEMATICAL SPACE
        | '\u{3000}' // IDEPGRAPHIC SPACE
        | '\u{FEFF}' // ZERO WIDTH NO-BREAK SPACE
            => CodeCharacterKind::HorizontalSpace,
        | '\n'       // LINE FEED
        | '\u{000B}' // LINE TABULATION
        | '\u{000C}' // FORM FEED
        | '\r'       // CARRIAGE RETURN
        | '\u{0085}' // NEXT LINE
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR 
            => CodeCharacterKind::VerticalSpace,
        | '!'  // EXCLAMATION MARK
        | '#'  // NUMBER SIGN
        | '$'  // DOLLAR SIGN
        | '%'  // PERCENT SIGN
        | '&'  // AMPERSAND
        | '*'  // ASTREISK
        | '+'  // PLUS SIGN
        | ','  // COMMA
        | '-'  // HYPHEN-MINUS
        | '.'  // FULL STOP
        | '/'  // SOLIDUS
        | ':'  // COLON
        | ';'  // SEMICOLON
        | '<'  // LESS-THAN SIGN
        | '='  // EQUALS SIGN
        | '>'  // GREATER-THAN SIGN
        | '?'  // QUESTION MARK
        | '@'  // COMMERCIAL AT
        | '\\' // REVERSE SOLIDUS
        | '^'  // CIRCUMFLEX ACCENT
        | '|'  // VERTICAL LINE
        | '~'  // TILDE
        | '('  // LEFT PARENTHESIS
        | '['  // LEFT SQUARE BRACKET
        | '{'  // LEFT CURLY BRACKET
        | ')'  // RIGHT PARENTHESIS
        | ']'  // RIGHT SQUARE BRACKET
        | '}'  // RIGHT CURLY BRACKET
            => CodeCharacterKind::Punctuation,
        _ => CodeCharacterKind::Text,
    }
}

pub(crate) fn is_vertical_space(c: char) -> bool {
    classifier_code_character(c) == CodeCharacterKind::VerticalSpace
}

pub(crate) fn is_punctuation(c: char) -> bool {
    classifier_code_character(c) == CodeCharacterKind::Punctuation
}

pub(crate) fn is_multispace(c: char) -> bool {
    matches!(
        classifier_code_character(c),
        CodeCharacterKind::VerticalSpace | CodeCharacterKind::HorizontalSpace
    )
}

pub(crate) fn is_text_char(c: char) -> bool {
    matches!(classifier_code_character(c), CodeCharacterKind::Text)
}
