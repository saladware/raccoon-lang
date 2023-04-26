pub enum Expr {
    Block(Box<Expr>),
    NumberLiteral(String),
    StringLiteral(String),
    Identifier(String),
    Seq(Vec<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Call()
}

pub struct Parser {
    statements: Vec<Expr>
}