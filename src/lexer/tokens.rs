#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub row: u64,
    pub column: u64,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String, row: u64, column: u64) -> Self {
        Self {
            kind,
            literal,
            row,
            column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    As,
    And,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Default,
    Do,
    Else,
    Enum,
    Extends,
    Finally,
    For,
    Function,
    Has,
    Hidden,
    If,
    InstanceOf,
    Import,
    Me,
    Module,
    Private,
    Protected,
    Public,
    Or,
    Return,
    /// I just added a random character at the
    /// end so the compiler won't complain
    /// because of Self keyword usage
    Self_,
    Static,
    Switch,
    Throw,
    Try,
    Using,
    Var,
    While,
    BoolLiteral,
    StringLiteral,
    IntLiteral,
    LongLiteral,
    FloatLiteral,
    DoubleLiteral,
    CharLiteral,
    DictionaryLiteral,
    ArrayLiteral,
    Null,
    Nan,
    New,
    Identifier,
    Semicolon,
    Comma,
    /// This one: {
    OpeningBracket,
    /// This one: }
    ClosingBracket,
    /// This one: (
    OpeningBrace,
    /// This one: )
    ClosingBrace,
    Asterisk,
    Percent,
    Assign,
    Bang,
    Tilde,
    Plus,
    Minus,
    Slash,
    /// This one: &
    Ampersand,
    LessThan,
    GreaterThan,
    /// This one: ^
    Caret,
    VerticalBar,
}
