use super::{Token, TokenType, Literal};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));
        return &self.tokens;
    }
    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            //is this idiomatic rust?
            '!' => {
                if self.check('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.check('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.check('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.check('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.check('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            _ => {
                //TODO is digit may have edge cases? c >= '0' && c <= '9';
                if c.is_digit(10) {
                    self.number();
                    //TODO can this be cleaned up using rust methods? edge cases using is alphabetic?
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    println!("{} Unexpected character", self.line)
                }
            }
        }
    }

    //TODO figure out how to use string literals in rust without errors instead of relying on the heap
    fn check_reserved(&mut self, s: String) -> TokenType {
        match s.as_str() {
            "and" => return TokenType::And,
            "class" => return TokenType::Class,
            "else" => return TokenType::Else,
            "false" => return TokenType::False,
            "for" => return TokenType::For,
            "fun" => return TokenType::Fun,
            "if" => return TokenType::If,
            "nil" => return TokenType::Nil,
            "or" => return TokenType::Or,
            "print" => return TokenType::Print,
            "return" => return TokenType::Return,
            "super" => return TokenType::Super,
            "this" => return TokenType::This,
            "true" => return TokenType::True,
            "var" => return TokenType::Var,
            "while" => return TokenType::While,
            _ => return TokenType::Identifier,
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = String::from(&self.source[self.start as usize..self.current as usize]);
        let token_type = self.check_reserved(text);
        self.add_token(token_type);
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let literal = Literal::Number(
            self.source[self.start as usize..self.current as usize]
                .parse()
                .unwrap(),
        );
        self.add_complete_token(TokenType::Number, Some(literal))
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth((self.current + 1) as usize)
            .unwrap();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        //TODO catch error for EOF with unterminated string

        self.advance();

        let literal = Literal::String(String::from(
            &self.source[(self.start + 1) as usize..(self.current - 1) as usize],
        ));
        //TODO add escaped sequences like \n
        self.add_complete_token(TokenType::String, Some(literal));
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn check(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn advance(&mut self) -> char {
        let chara = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        return chara;
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_complete_token(token_type, None);
    }

    fn add_complete_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = String::from(&self.source[self.start as usize..self.current as usize]);
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn is_at_end(&mut self) -> bool {
        return self.current as usize >= self.source.len();
    }
}

