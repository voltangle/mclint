#[derive(Debug)]
pub enum MonkeyCStatement {
    VariableCreation {
        name: String,
        default_val: Expression
    },
    ClassDeclaration {
        name: String,
        extends: Option<String>,
        children: Vec<MonkeyCStatement>
    }
    // Others will be added in later
}

#[derive(Debug)]
pub enum Expression {
    /// For "simple" assignment, like `var myVar = "simple string"`
    Simple(String)

}