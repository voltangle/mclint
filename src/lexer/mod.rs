use crate::lexer::tokens::{Token, TokenKind};

pub mod tokens;

#[derive(Debug)]
pub struct MonkeyCLexer {
    source: Vec<char>,
    currently_at: usize,
}

impl MonkeyCLexer {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source,
            currently_at: 0,
        }
    }

    pub fn lex(&mut self) {
        let mut tokens: Vec<Token> = Vec::new();
        while self.source.len() > self.currently_at {
            let c = self.current_char();

            match c {
                '{' => {
                    tokens.push(Token::new(TokenKind::OpeningBracket, c.to_string()));
                    self.currently_at += 1;
                }
                '}' => {
                    tokens.push(Token::new(TokenKind::ClosingBracket, c.to_string()));
                    self.currently_at += 1;
                }
                '(' => {
                    tokens.push(Token::new(TokenKind::OpeningBrace, c.to_string()));
                    self.currently_at += 1;
                }
                ')' => {
                    tokens.push(Token::new(TokenKind::ClosingBrace, c.to_string()));
                    self.currently_at += 1;
                }
                '~' => {
                    tokens.push(Token::new(TokenKind::Tilde, c.to_string()));
                    self.currently_at += 1;
                }
                '=' => {
                    tokens.push(Token::new(TokenKind::Assign, c.to_string()));
                    self.currently_at += 1;
                }
                '-' => {
                    tokens.push(Token::new(TokenKind::Minus, c.to_string()));
                    self.currently_at += 1;
                }
                '+' => {
                    tokens.push(Token::new(TokenKind::Plus, c.to_string()));
                    self.currently_at += 1;
                }
                '/' => {
                    tokens.push(Token::new(TokenKind::Slash, c.to_string()));
                    self.currently_at += 1;
                }
                ',' => {
                    tokens.push(Token::new(TokenKind::Comma, c.to_string()));
                    self.currently_at += 1;
                }
                ';' => {
                    tokens.push(Token::new(TokenKind::Semicolon, c.to_string()));
                    self.currently_at += 1;
                }
                '!' => {
                    tokens.push(Token::new(TokenKind::Bang, c.to_string()));
                    self.currently_at += 1;
                }
                '\'' => {
                    self.currently_at += 1;
                    tokens.push(Token::new(TokenKind::Char, self.current_char().to_string()));
                    self.currently_at += 2;
                }
                '\"' => {
                    self.currently_at += 1;
                    let mut buffer: String = String::new();
                    while self.current_char() != '\"' {
                        buffer.push(self.current_char());
                        self.currently_at += 1;
                    }
                    tokens.push(Token::new(TokenKind::String, buffer));
                    self.currently_at += 1;
                }
                _ => {
                    if c.is_alphabetic() {
                        // Creating a buffer and writing a first character to it
                        let mut buffer = String::new();
                        buffer.push(c);
                        self.currently_at += 1;

                        // Writing everything that is alphabetic to the buffer
                        while self.current_char().is_alphabetic() {
                            buffer.push(self.current_char());
                            self.currently_at += 1;
                        }

                        // Then matching for reserved words. If it's not reserved, then it's an identifier
                        let kind: TokenKind = match buffer.as_str() {
                            "as" => TokenKind::As,
                            "and" => TokenKind::And,
                            "break" => TokenKind::Break,
                            "case" => TokenKind::Case,
                            "catch" => TokenKind::Catch,
                            "class" => TokenKind::Class,
                            "const" => TokenKind::Const,
                            "continue" => TokenKind::Continue,
                            "default" => TokenKind::Default,
                            "do" => TokenKind::Do,
                            "else" => TokenKind::Else,
                            "enum" => TokenKind::Enum,
                            "extends" => TokenKind::Extends,
                            "finally" => TokenKind::Finally,
                            "for" => TokenKind::For,
                            "function" => TokenKind::Function,
                            "has" => TokenKind::Has,
                            "hidden" => TokenKind::Hidden,
                            "if" => TokenKind::If,
                            "instanceof" => TokenKind::InstanceOf,
                            "import" => TokenKind::Import,
                            "me" => TokenKind::Me,
                            "module" => TokenKind::Module,
                            "new" => TokenKind::New,
                            "null" => TokenKind::Null,
                            "NaN" => TokenKind::Nan,
                            "private" => TokenKind::Private,
                            "protected" => TokenKind::Protected,
                            "public" => TokenKind::Public,
                            "or" => TokenKind::Or,
                            "return" => TokenKind::Return,
                            "self" => TokenKind::SelfK,
                            "static" => TokenKind::Static,
                            "switch" => TokenKind::Switch,
                            "throw" => TokenKind::Throw,
                            "try" => TokenKind::Try,
                            "using" => TokenKind::Using,
                            "var" => TokenKind::Var,
                            "while" => TokenKind::While,
                            _ => TokenKind::Identifier,
                        };
                        tokens.push(Token::new(kind, buffer))
                    } else if c.is_numeric() {
                        // Creating a buffer and writing a first character to it
                        let mut buffer = String::new();
                        buffer.push(c);
                        let mut token_type = TokenKind::Int;

                        // Writing everything that is numeric or ./l/d to the buffer
                        while self.current_char().is_alphanumeric() || self.current_char() == '.' || self.current_char() == 'l' || self.current_char() == 'd' {
                            match self.current_char() {
                                '.' => {
                                    token_type = TokenKind::Float;
                                }
                                'l' => {
                                    token_type = TokenKind::Long;
                                }
                                'd' => {
                                    token_type = TokenKind::Double;
                                }
                                _ => {}
                            }
                            buffer.push(self.current_char());
                            self.currently_at += 1;
                        }
                        tokens.push(Token::new(token_type, buffer));
                    } else {
                        self.currently_at += 1;
                    }
                }
            }
        }
        println!("{:?}", tokens)
    }

    fn current_char(&self) -> char {
        *self.source.get(self.currently_at).unwrap()
    }
}
