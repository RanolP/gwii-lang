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
    pub name: String,
    pub particle: String,
}
