use crate::lexer::MonkeyCLexer;
use crate::parser::MonkeyCParser;
use anyhow::Context;
use anyhow::Result;
use crate::parser::ast::MonkeyCStatement::VariableDeclaration;
use crate::parser::ast::MonkeyCExpression;
use std::path::PathBuf;

#[test]
fn basic_code() -> Result<()> {
    let data = "var myVar as String = \"Hm\";\nvar myOtherVar = myVar;";
    let mut lexer = MonkeyCLexer::new(data.chars().collect());
    let tokens = lexer.lex().with_context(|| "Failed to tokenize data")?;
    println!("{:?}", tokens.clone());
    let mut parser = MonkeyCParser::new(tokens, PathBuf::from("<test>"));

    // Check for equality of parsed syntax
    assert_eq!(parser.parse().unwrap(), vec![VariableDeclaration { name: "myVar".to_string(), default_val: MonkeyCExpression::Simple("Hm".to_string()), var_type: Some("String".to_string()), is_const: false },
                                                                             VariableDeclaration { name: "myOtherVar".to_string(), default_val: MonkeyCExpression::Reference("myVar".to_string()), var_type: None, is_const: false }]);
    Ok(())
}
