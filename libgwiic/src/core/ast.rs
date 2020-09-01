use crate::core::Spanned;

#[derive(Debug)]
pub enum Node {
    Decl(Decl),
}

#[derive(Debug)]
pub enum Decl {
    Verb(VerbDecl),
}

#[derive(Debug)]
pub struct VerbDecl {
    pub is_extern: bool,
    pub parameters: Vec<Spanned<Parameter>>,
    pub name: String,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: ParticledName,
}

#[derive(Debug)]
pub struct ParticledName {
    pub name: String,
    pub particle: String,
}

#[derive(Debug)]
pub enum Atom {
    Integer(IntegerLiteralKind, String),
    Decimal(String),
    Character(String),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
pub enum IntegerLiteralKind {
    Binary,
    Octal,
    Decimal,
    Hex,
}
