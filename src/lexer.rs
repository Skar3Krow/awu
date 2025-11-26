use crate::tokens::Token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    chars: Vec<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            chars: input.chars().collect(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).cloned()
    }

    fn next(&mut self) -> Option<char> {
        if self.pos >= self.chars.len() {
            None
        } else {
            let ch = self.chars[self.pos];
            self.pos += 1;
            Some(ch)
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if Some(expected) == self.peek() {
            self.next();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }

    fn read_word(&mut self) -> Token {
        let mut buf = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_whitespace() || ['|', '&', '<', '>', '"', '\'', '(', ')'].contains(&ch) {
                break;
            }
            buf.push(ch);
            self.next();
        }
        Token::Word(buf)
    }

    fn read_single_quoted(&mut self) -> Token {
        self.next(); //Skip the first '

        let mut buf = String::new();
        while let Some(ch) = self.next() {
            match ch {
                '\'' => break,
                _ => buf.push(ch),
            }
        }
        Token::StringLiteral(buf)
    }

    fn read_double_quoted(&mut self) -> Token {
        self.next(); //Skip the first "

        let mut buf = String::new();
        while let Some(ch) = self.next() {
            match ch {
                '"' => break,
                '\\' => {
                    if let Some(next) = self.next() {
                        buf.push(next);
                    }
                }
                _ => buf.push(ch),
            }
        }
        Token::StringLiteral(buf)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let ch = match self.peek() {
            Some(c) => c,
            None => return Token::EOF,
        };
        match ch {
            '>' => {
                self.next();
                if self.match_next('>') {
                    return Token::RedirectAppend;
                }
                return Token::RedirectOut;
            }
            '<' => {
                self.next();
                return Token::RedirectIn;
            }
            '|' => {
                self.next();
                if self.match_next('|') {
                    return Token::Or;
                }
                return Token::Pipe;
            }
            '&' => {
                self.next();
                return Token::Background;
            }
            '(' => {
                self.next();
                return Token::LeftParen;
            }
            ')' => {
                self.next();
                return Token::RightParen;
            }

            '"' => return self.read_double_quoted(),
            '\'' => return self.read_single_quoted(),

            _ => return self.read_word(),
        }
    }
}
