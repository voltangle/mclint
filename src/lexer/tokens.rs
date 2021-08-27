#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    As,
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
    Instanceof,
    Import,
    Me,
    Module,
    Private,
    Protected,
    Public,
    Return,
    /// I just added a random character at the
    /// end so the compiler won't complain
    /// because of Self keyword usage
    SelfT,
    Static,
    Switch,
    Throw,
    Try,
    Using,
    Var,
    While,
    Bool,
    String,
    Int,
    Null,
    Char,
    Nan,
    New,
    Assign,
    Bang,
    Tilde,
    Identifier,
    Whitespace,
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
}

// Operator	Description
// new	creation
// !	logical NOT
// ~	bitwise NOT
// ( )	function invocation
// *	multiplication
// /	division
// %	modulo
// &	bitwise AND
// <<	left shift
// >>	right shift
// +	addition
// -	subtraction
// |	bitwise OR
// ^	bitwise XOR
// <	less than
// <=	less than or equals
// >	greater than
// >=	greater than or equals
// ==	equals
// !=	not equals
// &&	logical AND
// and
// ||	logical OR
// or
