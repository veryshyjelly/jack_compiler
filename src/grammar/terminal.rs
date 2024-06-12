#[derive(Debug, Eq, PartialEq)]
pub enum Terminal {
    Keyword(Keyword),
    Symbol(char),
    IntegerConstant(u16),
    StringConstant(String),
    Identifier(Identifier),
}

impl Terminal {
    pub fn keyword(self) -> Option<Keyword> {
        match self {
            Terminal::Keyword(val) => Some(val),
            _ => None,
        }
    }
    pub fn symbol(self) -> Option<char> {
        match self {
            Terminal::Symbol(val) => Some(val),
            _ => None,
        }
    }
    pub fn integer(self) -> Option<u16> {
        match self {
            Terminal::IntegerConstant(val) => Some(val),
            _ => None,
        }
    }
    pub fn string(self) -> Option<String> {
        match self {
            Terminal::StringConstant(val) => Some(val),
            _ => None,
        }
    }
    pub fn identifier(self) -> Option<Identifier> {
        match self {
            Terminal::Identifier(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

impl Keyword {
    pub fn from_str(word: &str) -> Option<Self> {
        use Keyword::*;
        let x = match word {
            "class" => Class,
            "constructor" => Constructor,
            "function" => Function,
            "method" => Method,
            "field" => Field,
            "static" => Static,
            "var" => Var,
            "int" => Int,
            "char" => Char,
            "boolean" => Boolean,
            "void" => Void,
            "true" => True,
            "false" => False,
            "null" => Null,
            "this" => This,
            "let" => Let,
            "do" => Do,
            "if" => If,
            "else" => Else,
            "while" => While,
            "return" => Return,
            _ => None?,
        };
        Some(x)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Identifier(pub String);
