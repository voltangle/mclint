use crate::lexer::MonkeyCLexer;
use crate::parser::ast::MonkeyCExpression;
use crate::parser::ast::MonkeyCStatement::VariableDeclaration;
use crate::parser::MonkeyCParser;
use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;

#[test]
fn basic_code() -> Result<()> {
    let data = "var myVar as String = \"Hm\";\nvar myOtherVar = myVar;";
    let mut lexer = MonkeyCLexer::new(data.chars().collect());
    let tokens = lexer.lex().with_context(|| "Failed to tokenize data")?;
    println!("{:?}", tokens.clone());
    let mut parser = MonkeyCParser::new(tokens, PathBuf::from("<test>"));

    // Check for equality of parsed syntax
    assert_eq!(
        parser.parse().unwrap(),
        vec![
            VariableDeclaration {
                name: Some("myVar".to_string()),
                default_val: Some(MonkeyCExpression::Simple("Hm".to_string())),
                var_type: Some("String".to_string()),
                is_const: false
            },
            VariableDeclaration {
                name: Some("myOtherVar".to_string()),
                default_val: Some(MonkeyCExpression::Reference("myVar".to_string())),
                var_type: None,
                is_const: false
            }
        ]
    );
    Ok(())
}
