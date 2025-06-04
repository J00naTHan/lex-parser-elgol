#[derive(Debug)]
pub enum Statement {
    VariableDeclaration {
        type_name: String,
        name: String,
    },
    Assignment {
        var_name: String,
        expression: Expression,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug)]
pub enum Expression {
    Number(i32),
    Variable(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    // Adicione outros operadores conforme necessário
}

#[derive(Debug)]
pub struct Function {
    pub return_type: String,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Parameter {
    pub type_name: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub main_block: Vec<Statement>,
}