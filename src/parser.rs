use crate::{
    lexer::{self, Lexer},
    tokens::Token,
};

#[derive(Debug)]
pub enum Command {
    Pipeline {
        commands: Vec<SimpleCommand>,
        background: bool,
    },
    Single(SimpleCommand),
}

#[derive(Debug)]
pub struct SimpleCommand {
    pub program: String,
    pub arg: Vec<String>,
    pub redirect_in: Option<String>,
    pub redirect_out: Option<String>,
    pub append: bool,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn expect(&mut self, token: Token) {
        if self.current == token {
            self.advance();
        } else {
            println!(
                "Unexpected Token: {:?}, Expected: {:?}",
                self.current, token
            );
        }
    }
}
