use crate::grammar::statement::Statements;
use crate::grammar::terminal::*;

pub struct Class(pub ClassName, pub Vec<ClassVarDec>, pub Vec<SubroutineDec>);
pub struct ClassVarDec(pub ClassVarType, pub Type, pub Vec<VarName>);

pub enum ClassVarType {
    Static,
    Field,
}

impl ClassVarType {
    pub fn from_keyword(tp: &Keyword) -> Option<Self> {
        use ClassVarType::*;
        match tp {
            Keyword::Static => Some(Static),
            Keyword::Field => Some(Field),
            _ => None,
        }
    }
}

pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(ClassName),
}

impl Type {
    pub fn from_terminal(tp: Terminal) -> Option<Self> {
        use Type::*;
        match tp {
            Terminal::Keyword(Keyword::Int) => Some(Int),
            Terminal::Keyword(Keyword::Char) => Some(Char),
            Terminal::Keyword(Keyword::Boolean) => Some(Char),
            Terminal::Identifier(val) => Some(ClassName(val)),
            _ => None,
        }
    }
}

pub struct SubroutineDec(
    pub SubroutineType,
    pub ReturnType,
    pub SubroutineName,
    pub ParameterList,
    pub SubroutineBody,
);

pub enum SubroutineType {
    Constructor,
    Function,
    Method,
}

impl SubroutineType {
    pub fn from_keyword(tp: &Keyword) -> Option<Self> {
        use SubroutineType::*;
        let x = match tp {
            Keyword::Constructor => Constructor,
            Keyword::Function => Function,
            Keyword::Method => Method,
            _ => None?,
        };
        Some(x)
    }
}

pub enum ReturnType {
    Base(Type),
    Void,
}

impl ReturnType {
    pub fn from_terminal(tp: Terminal) -> Option<Self> {
        use ReturnType::*;
        match tp {
            Terminal::Keyword(Keyword::Void) => Some(Void),
            tp => Some(Base(Type::from_terminal(tp)?)),
        }
    }
}

pub type ParameterList = Vec<Parameter>;
pub struct Parameter(pub Type, pub Identifier);
pub struct SubroutineBody(pub Vec<VarDec>, pub Statements);
pub struct VarDec(pub Type, pub Vec<VarName>);
pub type VarName = Identifier;
pub type SubroutineName = Identifier;
pub type ClassName = Identifier;
