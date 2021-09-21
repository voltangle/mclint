use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::ast::{MonkeyCStatement, MonkeyCExpression};
use crate::parser::err::MCParseError;
use std::path::PathBuf;

pub mod ast;
mod err;

macro_rules! syntax_expect_fmt {
    ($file:expr, $expected:expr, $actual:expr) => {
        format!(
            "Syntax error at {}:{}:{}: Unexpected token '{}', should've been {}",
            $file, $actual.row, $actual.column, $actual.literal, $expected
        );
    };
}

/// The same as syntax_expect_fmt, but without
/// a "Syntax error at <row>:<column>" at the
/// start
macro_rules! syntax_expect_fmt_headl {
    ($expected:expr, $actual:expr) => {
        format!(
            "Unexpected token '{}', should've been '{}'",
            $actual.literal, $expected
        );
    };
}

pub struct MonkeyCParser {
    token_list: Vec<Token>,
    file_path: PathBuf,
    currently_at: usize,
}

#[derive(Debug, Clone)]
struct ParserEnvironment {
    next_expected: Vec<TokenKind>,
    // For context saving, for example when we
    // are inside a class statement, we write
    // received data here
    context: Vec<MonkeyCStatement>,
}

const DEFAULT_EXPECT_TOKEN: [TokenKind; 39] = [
    TokenKind::As,
    TokenKind::And,
    TokenKind::Break,
    TokenKind::Case,
    TokenKind::Catch,
    TokenKind::Class,
    TokenKind::Const,
    TokenKind::Continue,
    TokenKind::Default,
    TokenKind::Do,
    TokenKind::Else,
    TokenKind::Enum,
    TokenKind::Extends,
    TokenKind::Finally,
    TokenKind::For,
    TokenKind::Function,
    TokenKind::Has,
    TokenKind::Hidden,
    TokenKind::If,
    TokenKind::InstanceOf,
    TokenKind::Import,
    TokenKind::Me,
    TokenKind::Module,
    TokenKind::New,
    TokenKind::Null,
    TokenKind::Nan,
    TokenKind::Private,
    TokenKind::Protected,
    TokenKind::Public,
    TokenKind::Or,
    TokenKind::Return,
    TokenKind::Self_,
    TokenKind::Static,
    TokenKind::Switch,
    TokenKind::Throw,
    TokenKind::Try,
    TokenKind::Using,
    TokenKind::Var,
    TokenKind::While,
];

impl ParserEnvironment {
    fn new() -> Self {
        Self {
            next_expected: Vec::from(DEFAULT_EXPECT_TOKEN),
            context: vec![],
        }
    }
}

fn nice_tk_vec_str(vec: Vec<TokenKind>) -> String {
    let mut str = String::new();

    for (index, token) in vec.iter().enumerate() {
        if index + 1 == vec.len() {
            str.push_str(&*format!("{}", token).to_lowercase())
        } else {
            str.push_str(&*format!("{}, ", token).to_lowercase())
        }
    }

    str
}

impl MonkeyCParser {
    pub fn new(token_list: Vec<Token>, file_path: PathBuf) -> Self {
        Self {
            token_list,
            file_path,
            currently_at: 0,
        }
    }

    /// Returns true if `k` in arguments is one of
    /// `BoolLiteral`, `StringLiteral`, `IntLiteral`,
    /// `LongLiteral`, `FloatLiteral`, `DoubleLiteral`,
    /// `Null`, or `CharLiteral`.
    fn is_kind_a_type(k: TokenKind) -> bool {
        if k == TokenKind::BoolLiteral
            || k == TokenKind::CharLiteral
            || k == TokenKind::StringLiteral
            || k == TokenKind::CharLiteral
            || k == TokenKind::LongLiteral
            || k == TokenKind::DoubleLiteral
            || k == TokenKind::FloatLiteral
            || k == TokenKind::IntLiteral
        {
            true
        } else {
            false
        }
    }

    fn current_token(&self) -> Token {
        self.token_list.get(self.currently_at).unwrap().clone()
    }

    //noinspection DuplicatedCode
    pub fn parse(&mut self) -> Result<Vec<MonkeyCStatement>, Vec<MCParseError>> {
        let mut statements: Vec<MonkeyCStatement> = Vec::new();
        let mut errors: Vec<MCParseError> = Vec::new();

        let mut environ = ParserEnvironment::new();

        while self.token_list.len() > self.currently_at {
            let t = self.current_token();
            self.currently_at += 1;
            if !environ.next_expected.contains(&t.kind) {
                errors.push(MCParseError {
                    at: (t.row, t.column),
                    literal_len: t.literal.len(),
                    full_msg: syntax_expect_fmt!(
                        self.file_path.clone().into_os_string().to_str().unwrap(),
                        format!("any of {}", nice_tk_vec_str(environ.next_expected.clone())),
                        t
                    ),
                    msg: syntax_expect_fmt_headl!(format!("{:?}", environ.next_expected), t),
                });
                continue;
            }
            match t.kind {
                TokenKind::And => {}
                TokenKind::As => environ.next_expected = vec![TokenKind::Identifier],
                TokenKind::Break => {}
                TokenKind::Case => {}
                TokenKind::Catch => {}
                TokenKind::Class => {}
                TokenKind::Const => {
                    environ.context.push(MonkeyCStatement::VariableDeclaration {
                        name: None,
                        default_val: None,
                        var_type: None,
                        is_const: true,
                    });
                    environ.next_expected = vec![TokenKind::Identifier]
                }
                TokenKind::Continue => {}
                TokenKind::Default => {}
                TokenKind::Do => {}
                TokenKind::Else => {}
                TokenKind::Enum => {}
                TokenKind::Extends => {}
                TokenKind::Finally => {}
                TokenKind::For => {}
                TokenKind::Function => {}
                TokenKind::Has => {}
                TokenKind::Hidden => {}
                TokenKind::If => {}
                TokenKind::InstanceOf => {}
                TokenKind::Import => {}
                TokenKind::Me => {}
                TokenKind::Module => {}
                TokenKind::Private => {}
                TokenKind::Protected => {}
                TokenKind::Public => {}
                TokenKind::Or => {}
                TokenKind::Return => {}
                TokenKind::Self_ => {}
                TokenKind::Static => {}
                TokenKind::Switch => {}
                TokenKind::Throw => {}
                TokenKind::Try => {}
                TokenKind::Using => {}
                TokenKind::Var => {
                    environ.context.push(MonkeyCStatement::VariableDeclaration {
                        name: None,
                        default_val: None,
                        var_type: None,
                        is_const: false,
                    });
                    environ.next_expected = vec![TokenKind::Identifier]
                }
                TokenKind::While => {}
                TokenKind::BoolLiteral => {}
                TokenKind::StringLiteral => {}
                TokenKind::IntLiteral => {}
                TokenKind::LongLiteral => {}
                TokenKind::FloatLiteral => {}
                TokenKind::DoubleLiteral => {}
                TokenKind::Null => {}
                TokenKind::CharLiteral => {}
                TokenKind::Nan => {}
                TokenKind::New => {}
                TokenKind::Identifier => {
                    if environ.context.is_empty() {
                        errors.push(MCParseError {
                            at: (t.row, t.column),
                            literal_len: t.literal.len(),
                            full_msg: syntax_expect_fmt!(
                                self.file_path.clone().into_os_string().to_str().unwrap(),
                                format!("any of {:?}", environ.next_expected),
                                t
                            ),
                            msg: syntax_expect_fmt_headl!(
                                format!("any of {:?}", environ.next_expected),
                                t
                            ),
                        })
                    } else {
                        let len = environ.clone().context.len() - 1;
                        match environ.context[len].clone() {
                            MonkeyCStatement::VariableDeclaration {
                                name,
                                default_val,
                                var_type,
                                is_const,
                            } => {
                                if name == None {
                                    environ.context.pop();
                                    environ.context.push(MonkeyCStatement::VariableDeclaration {
                                        name: Some(t.literal),
                                        default_val: None,
                                        var_type: None,
                                        is_const,
                                    });
                                    environ.next_expected = vec![TokenKind::As, TokenKind::Assign, TokenKind::Semicolon]
                                } else if var_type == None {
                                    environ.context.pop();
                                    environ.context.push(MonkeyCStatement::VariableDeclaration {
                                        name,
                                        default_val,
                                        var_type: Some(t.literal),
                                        is_const,
                                    });
                                    environ.next_expected = vec![TokenKind::Assign]
                                } else if default_val == None {
                                    environ.context.pop();
                                    environ.context.push(MonkeyCStatement::VariableDeclaration {
                                        name,
                                        default_val: Some(MonkeyCExpression::Simple(t.literal)),
                                        var_type,
                                        is_const,
                                    });
                                    environ.next_expected = vec![TokenKind::Semicolon]
                                }
                            }
                            MonkeyCStatement::ClassDeclaration { .. } => {}
                            MonkeyCStatement::EnumDeclaration { .. } => {}
                        }
                    }
                }
                // Punctuation
                TokenKind::Comma => {}
                TokenKind::OpeningBracket => {}
                TokenKind::ClosingBracket => {}
                TokenKind::OpeningBrace => {}
                TokenKind::ClosingBrace => {}
                TokenKind::Asterisk => {}
                TokenKind::Percent => {}
                TokenKind::Assign => {
                    environ.next_expected = vec![TokenKind::Identifier]
                },
                TokenKind::Bang => {}
                TokenKind::Tilde => {}
                TokenKind::Plus => {}
                TokenKind::Minus => {}
                TokenKind::Slash => {}
                TokenKind::Ampersand => {}
                TokenKind::LessThan => {}
                TokenKind::GreaterThan => {}
                TokenKind::Caret => {}
                TokenKind::VerticalBar => {}
                TokenKind::Semicolon => {
                    match &*environ.context.last().unwrap() {
                        MonkeyCStatement::VariableDeclaration { .. } => {
                            statements.push(environ.context[environ.context.len() - 1].clone());
                        }
                        MonkeyCStatement::ClassDeclaration { name, extends, children } => {
                            statements.pop();
                            let mut class_children = children.clone();
                            class_children.push(environ.context[environ.context.len() - 1].clone());
                            statements.push(MonkeyCStatement::ClassDeclaration {
                                name: (*name.clone()).parse().unwrap(),
                                extends: extends.clone(),
                                children: vec![]
                            })
                        }
                        _ => {}
                    }
                    environ.next_expected = Vec::from(DEFAULT_EXPECT_TOKEN)
                }
                TokenKind::DictionaryLiteral => {}
                TokenKind::ArrayLiteral => {}
                TokenKind::OnelineComment => {}
                TokenKind::MultilineComment => {}
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(statements)
    }
}
