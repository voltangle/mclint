use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::syntax::{MonkeyCStatement, Expression};
use anyhow::bail;
use anyhow::Result;

pub mod syntax;

pub struct MonkeyCParser {
    token_list: Vec<Token>,
    currently_at: usize,
}

impl MonkeyCParser {
    pub fn new(token_list: Vec<Token>) -> Self {
        Self {
            token_list,
            currently_at: 0
        }
    }

    fn current_token(&self) -> Token {
        self.token_list.get(self.currently_at).unwrap().clone()
    }

    pub fn parse(&mut self) -> Result<Vec<MonkeyCStatement>> {
        let mut statements: Vec<MonkeyCStatement> = Vec::new();

        // This is for understanding whether we finished to parse a line
        let mut line_finished = true;
        while self.token_list.len() > self.currently_at {
            let t = self.current_token();

            match t.kind {
                TokenKind::And => {
                    self.currently_at += 1;
                },
                TokenKind::Break => {
                    self.currently_at += 1;
                },
                TokenKind::Case => {
                    self.currently_at += 1;
                },
                TokenKind::Catch => {
                    self.currently_at += 1;
                },
                TokenKind::Class => {
                    self.currently_at += 1;
                },
                TokenKind::Const => {
                    self.currently_at += 1;
                },
                TokenKind::Continue => {
                    self.currently_at += 1;
                },
                TokenKind::Default => {
                    self.currently_at += 1;
                },
                TokenKind::Do => {
                    self.currently_at += 1;
                },
                TokenKind::Else => {
                    self.currently_at += 1;
                },
                TokenKind::Enum => {
                    self.currently_at += 1;
                },
                TokenKind::Extends => {
                    self.currently_at += 1;
                },
                TokenKind::Finally => {
                    self.currently_at += 1;
                },
                TokenKind::For => {
                    self.currently_at += 1;
                },
                TokenKind::Function => {
                    self.currently_at += 1;
                },
                TokenKind::Has => {
                    self.currently_at += 1;
                },
                TokenKind::Hidden => {
                    self.currently_at += 1;
                },
                TokenKind::If => {
                    self.currently_at += 1;
                },
                TokenKind::InstanceOf => {
                    self.currently_at += 1;
                },
                TokenKind::Import => {
                    self.currently_at += 1;
                },
                TokenKind::Me => {
                    self.currently_at += 1;
                },
                TokenKind::Module => {
                    self.currently_at += 1;
                },
                TokenKind::Private => {
                    self.currently_at += 1;
                },
                TokenKind::Protected => {
                    self.currently_at += 1;
                },
                TokenKind::Public => {
                    self.currently_at += 1;
                },
                TokenKind::Or => {
                    self.currently_at += 1;
                },
                TokenKind::Return => {
                    self.currently_at += 1;
                },
                TokenKind::SelfK => {
                    self.currently_at += 1;
                },
                TokenKind::Static => {
                    self.currently_at += 1;
                },
                TokenKind::Switch => {
                    self.currently_at += 1;
                },
                TokenKind::Throw => {
                    self.currently_at += 1;
                },
                TokenKind::Try => {
                    self.currently_at += 1;
                },
                TokenKind::Using => {
                    self.currently_at += 1;
                },
                TokenKind::Var => {
                    let mut name: String = String::new();
                    let mut var_type: Option<String> = None;
                    let mut default_val: Expression = Expression::Simple("".to_string());

                    self.currently_at += 1;
                    if !line_finished {
                        bail!(format!("Syntax error at {}:{}: Expected at least ';' token, found '{}'", self.current_token().row, self.current_token().column, self.current_token().literal));
                    }
                    line_finished = false;
                    if self.current_token().kind != TokenKind::Identifier {
                        bail!(format!("Syntax error at {}:{}: Expected an identifier, found '{}'", self.current_token().row, self.current_token().column, self.current_token().literal));
                    }
                    name = self.current_token().literal;
                    self.currently_at += 1;
                    match self.current_token().kind {
                        TokenKind::Assign => {
                            self.currently_at += 1;
                            let k = self.current_token().kind;
                            if k == TokenKind::Identifier || k == TokenKind::BoolLiteral || k == TokenKind::CharLiteral || k == TokenKind::StringLiteral || k == TokenKind::CharLiteral || k == TokenKind::LongLiteral || k == TokenKind::DoubleLiteral || k == TokenKind::FloatLiteral || k == TokenKind::IntLiteral {} else {
                                bail!(format!("Syntax error at {}:{}: Expected an identifier, found '{}'", self.current_token().row, self.current_token().column, self.current_token().literal));
                            }
                        }
                        TokenKind::As => {
                            self.currently_at += 1;
                            if self.current_token().kind != TokenKind::Identifier {
                                bail!(format!("Syntax error at {}:{}: Expected an identifier after 'as', found '{}'", self.current_token().row, self.current_token().column, self.current_token().literal));
                            }
                            var_type = Some(self.current_token().literal);
                            self.currently_at += 2;
                            let k = self.current_token().kind;
                            if k == TokenKind::Identifier || k == TokenKind::BoolLiteral || k == TokenKind::CharLiteral || k == TokenKind::StringLiteral || k == TokenKind::CharLiteral || k == TokenKind::LongLiteral || k == TokenKind::DoubleLiteral || k == TokenKind::FloatLiteral || k == TokenKind::IntLiteral {} else {
                                bail!(format!("Syntax error at {}:{}: Expected an identifier or literal, found '{}'", self.current_token().row, self.current_token().column, self.current_token().literal));
                            }
                        }
                        _ => {
                            bail!(format!("Syntax error at {}:{}: Expected an '=' token or type declaration, found '{}'", self.current_token().row, self.current_token().column, self.current_token().literal));
                        }
                    }
                    default_val = Expression::Simple(self.current_token().literal);
                    let statement = MonkeyCStatement::VariableDeclaration {
                        name,
                        default_val,
                        var_type
                    };
                    statements.push(statement);
                    self.currently_at += 1;
                }
                TokenKind::While => {
                    self.currently_at += 1;
                },
                TokenKind::BoolLiteral => {
                    self.currently_at += 1;
                },
                TokenKind::StringLiteral => {
                    self.currently_at += 1;
                },
                TokenKind::IntLiteral => {
                    self.currently_at += 1;
                },
                TokenKind::LongLiteral => {
                    self.currently_at += 1;
                },
                TokenKind::FloatLiteral => {
                    self.currently_at += 1;
                },
                TokenKind::DoubleLiteral => {
                    self.currently_at += 1;
                },
                TokenKind::Null => {
                    self.currently_at += 1;
                },
                TokenKind::CharLiteral => {
                    self.currently_at += 1;
                },
                TokenKind::Nan => {
                    self.currently_at += 1;
                },
                TokenKind::New => {
                    self.currently_at += 1;
                },
                TokenKind::Identifier => {
                    self.currently_at += 1;
                },
                TokenKind::Comma => {
                    self.currently_at += 1;
                },
                TokenKind::OpeningBracket => {
                    self.currently_at += 1;
                },
                TokenKind::ClosingBracket => {
                    self.currently_at += 1;
                },
                TokenKind::OpeningBrace => {
                    self.currently_at += 1;
                },
                TokenKind::ClosingBrace => {
                    self.currently_at += 1;
                },
                TokenKind::Asterisk => {
                    self.currently_at += 1;
                },
                TokenKind::Percent => {
                    self.currently_at += 1;
                },
                TokenKind::Assign => {
                    self.currently_at += 1;
                },
                TokenKind::Bang => {
                    self.currently_at += 1;
                },
                TokenKind::Tilde => {
                    self.currently_at += 1;
                },
                TokenKind::Plus => {
                    self.currently_at += 1;
                },
                TokenKind::Minus => {
                    self.currently_at += 1;
                },
                TokenKind::Slash => {
                    self.currently_at += 1;
                },
                TokenKind::Ampersand => {
                    self.currently_at += 1;
                },
                TokenKind::LessThan => {
                    self.currently_at += 1;
                },
                TokenKind::GreaterThan => {
                    self.currently_at += 1;
                },
                TokenKind::Caret => {
                    self.currently_at += 1;
                },
                TokenKind::VerticalBar => {
                    self.currently_at += 1;
                },
                TokenKind::Semicolon => {
                    line_finished = true;
                    self.currently_at += 1;
                }
                _ => {
                    self.currently_at += 1;
                }
            }
        }
        println!("{:?}", statements);
        Ok(statements)
    }
}