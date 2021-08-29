use crate::lexer::MonkeyCLexer;
use crate::parser::MonkeyCParser;
use anyhow::Context;
use anyhow::Result;
use crate::parser::syntax::MonkeyCStatement::VariableCreation;
use crate::parser::syntax::Expression::Simple;

#[test]
fn basic_code() -> Result<()> {
    let data = "var myVar as String = \"Hm\";\nvar myOtherVar = myVar;";
    let mut lexer = MonkeyCLexer::new(data.chars().collect());
    let tokens = lexer.lex().with_context(|| "Failed to tokenize data")?;
    let mut parser = MonkeyCParser::new(tokens);

    // Check for equality of parsed syntax
    assert_eq!(parser.parse().with_context(|| "Failed to parse data")?, vec![VariableCreation { name: "myVar".to_string(), default_val: Simple("Hm".to_string()), var_type: Some("String".to_string()) },
VariableCreation { name: "myOtherVar".to_string(), default_val: Simple("myVar".to_string()), var_type: None }]);
    Ok(())
}