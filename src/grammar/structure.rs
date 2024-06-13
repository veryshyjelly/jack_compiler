use crate::grammar::statement::Statements;
use crate::grammar::terminal::*;

pub struct Class(pub ClassName, pub Vec<ClassVarDec>, pub Vec<SubroutineDec>);
pub struct ClassVarDec(pub ClassVarType, pub Type, pub Vec<VarName>);

pub enum ClassVarType {
    Static,
    Field,
}

impl ClassVarType {
    pub fn from_keyword(keyword: &Keyword) -> Result<Self, String> {
        use ClassVarType::*;
        match keyword {
            Keyword::Static => Ok(Static),
            Keyword::Field => Ok(Field),
            e => Err(format!("expected static or field found: {e:?}")),
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
    pub fn from_terminal(terminal: Terminal) -> Result<Self, String> {
        use Type::*;
        match terminal {
            Terminal::Keyword(Keyword::Int) => Ok(Int),
            Terminal::Keyword(Keyword::Char) => Ok(Char),
            Terminal::Keyword(Keyword::Boolean) => Ok(Char),
            Terminal::Identifier(val) => Ok(ClassName(val)),
            e => Err(format!("type expected found: {e:?}")),
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
    pub fn from_keyword(keyword: &Keyword) -> Result<Self, String> {
        use SubroutineType::*;
        let x = match keyword {
            Keyword::Constructor => Constructor,
            Keyword::Function => Function,
            Keyword::Method => Method,
            e => Err(format!("constructor|function|method expected found: {e:?}"))?,
        };
        Ok(x)
    }
}

pub enum ReturnType {
    Base(Type),
    Void,
}

impl ReturnType {
    pub fn from_terminal(tp: Terminal) -> Result<Self, String> {
        use ReturnType::*;
        match tp {
            Terminal::Keyword(Keyword::Void) => Ok(Void),
            tp => Ok(Base(Type::from_terminal(tp)?)),
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
