#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Terminal {
    Keyword(Keyword),
    Symbol(char),
    IntegerConstant(u16),
    StringConstant(String),
    Identifier(Identifier),
}

impl Terminal {
    pub fn keyword(self) -> Result<Keyword, String> {
        match self {
            Terminal::Keyword(val) => Ok(val),
            e => Err(format!("keyword expected found: {e:?}")),
        }
    }
    pub fn symbol(self) -> Result<char, String> {
        match self {
            Terminal::Symbol(val) => Ok(val),
            e => Err(format!("symbol expected found: {e:?}")),
        }
    }
    pub fn integer(self) -> Result<u16, String> {
        match self {
            Terminal::IntegerConstant(val) => Ok(val),
            e => Err(format!("integer expected found: {e:?}")),
        }
    }
    pub fn string(self) -> Result<String, String> {
        match self {
            Terminal::StringConstant(val) => Ok(val),
            e => Err(format!("string constant expected found: {e:?}")),
        }
    }
    pub fn identifier(self) -> Result<Identifier, String> {
        match self {
            Terminal::Identifier(val) => Ok(val),
            e => Err(format!("identifier expected found: {e:?}")),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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
    pub fn from_str(word: &str) -> Result<Self, String> {
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
            e => Err(format!("keyword expected found: {e:?}"))?,
        };
        Ok(x)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Identifier(pub String);
