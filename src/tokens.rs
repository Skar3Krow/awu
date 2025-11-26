#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    StringLiteral(String),
    Pipe,
    And,
    Or,
    RedirectOut,
    RedirectAppend,
    RedirectIn,
    Background,
    LeftParen,
    RightParen,
    EOF,
}
