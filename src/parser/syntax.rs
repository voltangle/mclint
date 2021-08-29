#[derive(Debug, PartialEq)]
pub enum MonkeyCStatement {
    VariableCreation {
        name: String,
        default_val: Expression,
        var_type: Option<String>,
    },
    ClassDeclaration {
        name: String,
        extends: Option<String>,
        children: Vec<MonkeyCStatement>,
    }
    // Others will be added in later
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    /// For "simple" assignment, like `var myVar = "simple string"`
    Simple(String)

}