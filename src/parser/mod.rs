use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::ast::MonkeyCStatement;
use crate::parser::err::MCParseError;
use std::path::PathBuf;

pub mod ast;
mod err;

macro_rules! syntax_expect_fmt {
    ($file:expr, $expected:expr, $actual:expr) => {
        format!(
            "Syntax error at {}:{}:{}: Expected {}, found '{}'",
            $file, $actual.row, $actual.column, $expected, $actual.literal
        );
    };
}

/// The same as syntax_expect_fmt, but without
/// a "Syntax error at <row>:<column>" at the
/// start
macro_rules! syntax_expect_fmt_headl {
    ($expected:expr, $actual:expr) => {
        format!("Expected {}, found '{}'", $expected, $actual.literal);
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

impl ParserEnvironment {
    fn new() -> Self {
        Self {
            next_expected: vec![
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
            ],
            context: vec![],
        }
    }
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
            if !environ.next_expected.contains(&t.kind) {
                errors.push(MCParseError {
                    at: (t.row, t.column),
                    literal_len: t.literal.len(),
                    full_msg: syntax_expect_fmt!(
                        self.file_path.clone().into_os_string().to_str().unwrap(),
                        format!("any of {:?}", environ.next_expected),
                        t
                    ),
                    msg: syntax_expect_fmt_headl!(format!("{:?}", environ.next_expected), t),
                });
                self.currently_at += 1;
                continue;
            }
            match t.kind {
                TokenKind::And => {}
                TokenKind::Break => {}
                TokenKind::Case => {}
                TokenKind::Catch => {}
                TokenKind::Class => {}
                TokenKind::Const => {}
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
                        let len = environ.clone().context.len();
                        match environ.context[len - 1] {
                            MonkeyCStatement::VariableDeclaration { is_const, .. } => {
                                environ.context.pop();
                                environ.context.push(MonkeyCStatement::VariableDeclaration {
                                    name: Some(t.literal),
                                    default_val: None,
                                    var_type: None,
                                    is_const,
                                });
                            }
                            MonkeyCStatement::ClassDeclaration { .. } => {}
                            MonkeyCStatement::EnumDeclaration { .. } => {}
                        }
                    }
                }
                TokenKind::Comma => {}
                TokenKind::OpeningBracket => {}
                TokenKind::ClosingBracket => {}
                TokenKind::OpeningBrace => {}
                TokenKind::ClosingBrace => {}
                TokenKind::Asterisk => {}
                TokenKind::Percent => {}
                TokenKind::Assign => {}
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
                TokenKind::Semicolon => {}
                TokenKind::As => {}
                TokenKind::DictionaryLiteral => {}
                TokenKind::ArrayLiteral => {}
            }
            self.currently_at += 1;
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(statements)
    }
}
