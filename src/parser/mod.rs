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
        println!("{:?}", statements);

        // This is for understanding whether we finished to parse a line
        let mut line_finished = true;
        while self.token_list.len() > self.currently_at {
            let t = self.current_token();

            match t.kind {
                TokenKind::As => unimplemented!(),
                TokenKind::And => unimplemented!(),
                TokenKind::Break => unimplemented!(),
                TokenKind::Case => unimplemented!(),
                TokenKind::Catch => unimplemented!(),
                TokenKind::Class => unimplemented!(),
                TokenKind::Const => unimplemented!(),
                TokenKind::Continue => unimplemented!(),
                TokenKind::Default => unimplemented!(),
                TokenKind::Do => unimplemented!(),
                TokenKind::Else => unimplemented!(),
                TokenKind::Enum => unimplemented!(),
                TokenKind::Extends => unimplemented!(),
                TokenKind::Finally => unimplemented!(),
                TokenKind::For => unimplemented!(),
                TokenKind::Function => unimplemented!(),
                TokenKind::Has => unimplemented!(),
                TokenKind::Hidden => unimplemented!(),
                TokenKind::If => unimplemented!(),
                TokenKind::InstanceOf => unimplemented!(),
                TokenKind::Import => unimplemented!(),
                TokenKind::Me => unimplemented!(),
                TokenKind::Module => unimplemented!(),
                TokenKind::Private => unimplemented!(),
                TokenKind::Protected => unimplemented!(),
                TokenKind::Public => unimplemented!(),
                TokenKind::Or => unimplemented!(),
                TokenKind::Return => unimplemented!(),
                TokenKind::SelfK => unimplemented!(),
                TokenKind::Static => unimplemented!(),
                TokenKind::Switch => unimplemented!(),
                TokenKind::Throw => unimplemented!(),
                TokenKind::Try => unimplemented!(),
                TokenKind::Using => unimplemented!(),
                TokenKind::Var => {
                    self.currently_at += 1;
                    if !line_finished {
                        bail!(format!("Syntax error at {}:{}: Expected at least ';' token, found '{}'", self.current_token().row, self.current_token().column, self.current_token().literal));
                    }
                    line_finished = false;
                    let statement = MonkeyCStatement::VariableCreation {
                        name: self.current_token().literal.to_string(),
                        default_val: {
                            self.currently_at += 1;
                            if self.current_token().kind != TokenKind::Assign {
                                bail!(format!("Syntax error at {}:{}: Expected '=' token, found '{}'", self.current_token().row, self.current_token().column, self.current_token().literal));
                            }
                            self.currently_at += 1;
                            Expression::Simple(self.current_token().literal)
                        }
                    };
                    println!("{:?}", statement);
                    statements.push(statement);
                    self.currently_at += 1;
                }
                TokenKind::While => unimplemented!(),
                TokenKind::Bool => unimplemented!(),
                TokenKind::String => unimplemented!(),
                TokenKind::Int => unimplemented!(),
                TokenKind::Long => unimplemented!(),
                TokenKind::Float => unimplemented!(),
                TokenKind::Double => unimplemented!(),
                TokenKind::Null => unimplemented!(),
                TokenKind::Char => unimplemented!(),
                TokenKind::Nan => unimplemented!(),
                TokenKind::New => unimplemented!(),
                TokenKind::Identifier => unimplemented!(),
                TokenKind::Comma => unimplemented!(),
                TokenKind::OpeningBracket => unimplemented!(),
                TokenKind::ClosingBracket => unimplemented!(),
                TokenKind::OpeningBrace => unimplemented!(),
                TokenKind::ClosingBrace => unimplemented!(),
                TokenKind::Asterisk => unimplemented!(),
                TokenKind::Percent => unimplemented!(),
                TokenKind::Assign => unimplemented!(),
                TokenKind::Bang => unimplemented!(),
                TokenKind::Tilde => unimplemented!(),
                TokenKind::Plus => unimplemented!(),
                TokenKind::Minus => unimplemented!(),
                TokenKind::Slash => unimplemented!(),
                TokenKind::Ampersand => unimplemented!(),
                TokenKind::LessThan => unimplemented!(),
                TokenKind::GreaterThan => unimplemented!(),
                TokenKind::Caret => unimplemented!(),
                TokenKind::VerticalBar => unimplemented!(),
                TokenKind::Semicolon => {
                    line_finished = true;
                    self.currently_at += 1;
                }
            }
        }
        Ok(statements)
    }
}