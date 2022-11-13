// NodeTypes: "Program", "NumericLiteral", "Identifier", "BinaryExp"

#[derive(Debug)]
pub enum StatementOrExpression {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Statement {
    Program(Program),
    VariableDecleration(VariableDecleration),
}

#[derive(Debug)]
pub enum Expression {
    NumericLiteral(NumericLiteral),
    Identifier(Identifier),
    BinaryExpr(BinaryExpr),
}


pub trait Stmt {
    fn node_type(&self) -> String;
}

pub trait Expr {}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<StatementOrExpression>,
}

impl Stmt for Program {
    fn node_type(&self) -> String {
        "Program".to_string()
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

impl Expr for BinaryExpr {}

impl Stmt for BinaryExpr {
    fn node_type(&self) -> String {
        "BinaryExp".to_string()
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub symbol: String,
}

impl Expr for Identifier {}

impl Stmt for Identifier {
    fn node_type(&self) -> String {
        "Identifier".to_string()
    }
}

#[derive(Debug)]
pub struct NumericLiteral {
    pub value: f64,
}

impl Expr for NumericLiteral {}

impl Stmt for NumericLiteral {
    fn node_type(&self) -> String {
        "NumericLiteral".to_string()
    }
}

#[derive(Debug)]
pub struct VariableDecleration {
    pub(crate) constant: bool,
    pub(crate) identifier: Identifier,
    pub(crate) value: Option<Expression>,
}

impl VariableDecleration {
    pub fn new(identifier: String, value: Option<Expression>, constant: bool) -> Self {
        VariableDecleration {
            constant,
            identifier: Identifier { symbol: identifier },
            value,
        }
    }
}

impl Stmt for VariableDecleration {
    fn node_type(&self) -> String {
        "VariableDecleration".to_string()
    }
}
