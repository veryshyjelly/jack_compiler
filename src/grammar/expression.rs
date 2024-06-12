use crate::grammar::structure::{ClassName, SubroutineName, VarName};

type ExpressionList = Vec<Expression>;

pub enum Expression {
    Term(Term),
    OpTerm(Vec<OpTerm>),
}

pub enum Term {
    IntegerConstant(u16),
    StringConstant(String),
    KeywordConstant(KeywordConstant),
    VarName(VarName),
    VarNameIndex(VarName, Box<Expression>),
    BracketExpression(Box<Expression>),
    UnaryOpTerm(UnaryOp, Box<Term>),
    SubroutineCall(SubroutineCall),
}

pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

pub struct SubroutineCall(Option<ClassOrVarName>, SubroutineName, ExpressionList);
enum ClassOrVarName {
    ClassName(ClassName),
    VarName(VarName),
}

pub struct OpTerm(Op, Term);

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Lt,
    Gt,
    Eq,
}

impl Op {
    pub fn from_char(c: char) -> Option<Self> {
        use Op::*;
        let x = match c {
            '+' => Add,
            '-' => Sub,
            '*' => Mul,
            '/' => Div,
            '&' => And,
            '|' => Or,
            '<' => Lt,
            '>' => Gt,
            '=' => Eq,
           _ => None?
        };
        Some(x)
    }
}

pub enum UnaryOp {
    Minus,
    Not,
}

impl UnaryOp {
    pub fn from_char(c: char) -> Option<Self> {
        use UnaryOp::*;
        match c {
            '-' => Some(Minus),
            '~' => Some(Not),
            _ => None
        }
    }
}
