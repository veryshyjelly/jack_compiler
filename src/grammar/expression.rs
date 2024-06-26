use crate::grammar::structure::{SubroutineName, VarName};
use crate::grammar::terminal::{Identifier, Keyword};

pub type ExpressionList = Vec<Expression>;

#[derive(Debug)]
pub struct Expression(pub Term, pub Vec<OpTerm>);

#[derive(Debug)]
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
#[derive(Debug)]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

impl KeywordConstant {
    pub fn from_keyword(keyword: Keyword) -> Result<Self, String> {
        use KeywordConstant::*;
        let res = match keyword {
            Keyword::True => True,
            Keyword::False => False,
            Keyword::Null => Null,
            Keyword::This => This,
            e => Err(format!(
                "expected true|false|null|this expected found: {e:?}"
            ))?,
        };
        Ok(res)
    }
}

#[derive(Debug)]
pub struct SubroutineCall(
    pub Option<ClassOrVarName>,
    pub SubroutineName,
    pub ExpressionList,
);
type ClassOrVarName = Identifier;

#[derive(Debug)]
pub struct OpTerm(pub Op, pub Term);

#[derive(Debug)]
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
            _ => None?,
        };
        Some(x)
    }
}

#[derive(Debug)]
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
            _ => None,
        }
    }
}
