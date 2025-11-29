use std::panic;

use crate::{lexer::Lexer, tokens::Token};

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

    fn parse_simple_command(&mut self) -> SimpleCommand {
        let mut program = None;
        let mut args = Vec::new();
        let mut redirect_in = None;
        let mut redirect_out = None;
        let mut append = false;

        loop {
            match &self.current {
                Token::Word(w) => {
                    if program.is_none() {
                        program = Some(w.clone());
                    } else {
                        args.push(w.clone());
                    }
                    self.advance();
                }
                Token::StringLiteral(s) => {
                    if program.is_none() {
                        program = Some(s.clone());
                    } else {
                        args.push(s.clone());
                    }
                    self.advance();
                }
                Token::RedirectIn => {
                    self.advance();
                    if let Token::Word(file) = &self.current {
                        redirect_in = Some(file.clone());
                        self.advance();
                    } else {
                        panic!("Error: Expected File name after '<' ");
                    }
                }
                Token::RedirectOut => {
                    self.advance();
                    if let Token::Word(file) = &self.current {
                        redirect_out = Some(file.clone());
                        append = false;
                        self.advance();
                    } else {
                        panic!("Error: Expected File name after '>' ");
                    }
                }
                Token::RedirectAppend => {
                    self.advance();
                    if let Token::Word(file) = &self.current {
                        redirect_out = Some(file.clone());
                        append = true;
                        self.advance();
                    } else {
                        panic!("Error: Expected File name after '>>' ");
                    }
                }
                _ => break,
            }
        }
        SimpleCommand {
            program: program.expect("Error: No program found"),
            arg: args,
            redirect_in,
            redirect_out,
            append,
        }
    }

    fn parse_pipeline(&mut self) -> Command {
        let mut commands = vec![self.parse_simple_command()];
        while self.current == Token::Pipe {
            self.advance();
            commands.push(self.parse_simple_command());
        }
        let mut background = false;
        if self.current == Token::Background {
            background = true;
            self.advance();
        }
        if commands.len() == 1 {
            Command::Single(commands.remove(0))
        } else {
            Command::Pipeline {
                commands,
                background,
            }
        }
    }

    pub fn parse(&mut self) -> Command {
        let cmd = self.parse_pipeline();
        if self.current != Token::EOF {
            panic!("Unexpected Command: {:?}", self.current);
        }
        cmd
    }
}
