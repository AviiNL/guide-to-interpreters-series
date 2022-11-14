use crate::environment::Environment;

#[derive(Debug, Clone)]
pub enum StatementOrExpression {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Program(Program),
    VariableDecleration(VariableDecleration),
    FunctionDecleration(FunctionDecleration),
}

#[derive(Debug, Clone)]
pub enum Expression {
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    Identifier(Identifier),
    Binary(Binary),
    Assignment(Assignment),
    Property(Property),
    ObjectLiteral(ObjectLiteral),
    ArrayLiteral(ArrayLiteral),
    Member(MemberExpr),
    Call(CallExpr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<StatementOrExpression>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Identifier>,
    pub body: Vec<StatementOrExpression>,
    pub env: Environment,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct NumericLiteral {
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FunctionDecleration {
    pub(crate) identifier: Identifier,
    pub(crate) params: Vec<Identifier>,
    pub(crate) body: Vec<StatementOrExpression>,
}

impl FunctionDecleration {
    pub fn new(identifier: String, params: Vec<String>, body: Vec<StatementOrExpression>) -> Self {
        FunctionDecleration {
            identifier: Identifier { symbol: identifier },
            params: params.into_iter().map(|s| Identifier { symbol: s }).collect(),
            body,
        }
    }

    pub fn get_name(&self) -> String {
        self.identifier.symbol.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub(crate) assignee: Box<Expression>,
    pub(crate) value: Box<Expression>,
}

impl Assignment {
    pub fn new(assignee: Expression, value: Expression) -> Self {
        Assignment {
            assignee: Box::new(assignee),
            value: Box::new(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    pub(crate) key: String,
    pub(crate) value: Option<Box<Expression>>,
}

impl Property {
    pub fn new(key: String, value: Expression) -> Self {
        Property {
            key,
            value: Some(Box::new(value)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectLiteral {
    pub(crate) properties: Vec<Property>,
}

impl ObjectLiteral {
    pub fn new(properties: Vec<Property>) -> Self {
        ObjectLiteral { properties }
    }
}

// implement iterator for ObjectLiteral
impl IntoIterator for ObjectLiteral {
    type Item = Property;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.properties.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub(crate) elements: Vec<Expression>,
}

impl ArrayLiteral {
    pub fn new(elements: Vec<Expression>) -> Self {
        ArrayLiteral { elements }
    }
}

// implement iterator for ArrayLiteral
impl IntoIterator for ArrayLiteral {
    type Item = Expression;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub(crate) caller: Box<Expression>,
    pub(crate) arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct MemberExpr {
    pub(crate) object: Box<Expression>,
    pub(crate) property: Box<Expression>,
    pub(crate) computed: bool,
}
