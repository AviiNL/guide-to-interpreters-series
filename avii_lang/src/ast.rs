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
    StringLiteral(StringLiteral),
    Identifier(Identifier),
    Binary(Binary),
    Assignment(Assignment),
    Property(Property),
    ObjectLiteral(ObjectLiteral),
    Member(MemberExpr),
    Call(CallExpr)
}

#[derive(Debug)]
pub struct Program {
    pub body: Vec<StatementOrExpression>,
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct Identifier {
    pub symbol: String,
}

#[derive(Debug)]
pub struct NumericLiteral {
    pub value: f64,
}

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CallExpr {
    pub(crate) caller: Box<Expression>,
    pub(crate) arguments: Vec<Expression>,
}

#[derive(Debug)]
pub struct MemberExpr {
    pub(crate) object: Box<Expression>,
    pub(crate) property: Box<Expression>,
    pub(crate) computed: bool,
}