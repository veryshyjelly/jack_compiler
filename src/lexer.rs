use crate::grammar::terminal;
use crate::grammar::terminal::Terminal;
use crate::grammar::terminal::Terminal::{
    Identifier, IntegerConstant, Keyword, StringConstant, Symbol,
};

pub struct Lexer<'a> {
    pub content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    // Trim whitespaces from left
    pub fn trim_left(&mut self) {
        if let Some(n) = self.content.iter().position(|x| !x.is_whitespace()) {
            self.content = &self.content[n..]
        } else {
            self.content = &[]
        }
    }

    // Chop n characters and return the token
    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[..n];
        self.content = &self.content[n..];
        token
    }

    // Chop while the given predicate is true
    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        if let Some(n) = self.content.iter().position(|x| !predicate(x)) {
            self.chop(n)
        } else {
            self.chop(self.content.len())
        }
    }

    // Get the next token ignoring the comments
    pub fn next_token(&mut self) -> Option<&'a [char]> {
        loop {
            // Ignore the comments
            self.trim_left();
            if self.content.len() > 1 && self.content[0] == '/' && self.content[1] == '/' {
                self.chop_while(|&x| !x.is_control());
                self.trim_left()
            } else if self.content.len() > 1 && self.content[0] == '/' && self.content[1] == '*' {
                let mut i = 3;
                while i < self.content.len()
                    && (self.content[i] != '/' || self.content[i - 1] != '*')
                {
                    i += 1;
                }
                if i + 1 > self.content.len() {
                    self.content = &[]
                } else {
                    self.content = &self.content[i + 1..]
                }
            } else {
                break;
            }
        }

        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            Some(self.chop_while(|x| x.is_numeric()))
        } else if self.content[0].is_alphabetic() {
            // Variable rules: can contain _
            Some(self.chop_while(|&x| x.is_alphanumeric() || x == '_'))
        } else {
            Some(self.chop(1))
        }
    }

    pub fn next_element(&mut self) -> Option<Terminal> {
        let token = self.next_token()?.into_iter().collect::<String>();
        let first_char = token.chars().nth(0)?;

        let term = if first_char.is_numeric() {
            IntegerConstant(token.parse().ok()?)
        } else if first_char.is_alphanumeric() {
            if let Ok(keyword) = terminal::Keyword::from_str(&token) {
                Keyword(keyword)
            } else {
                Identifier(terminal::Identifier(token))
            }
        } else if first_char == '"' {
            let st = self
                .chop_while(|&x| x != '"')
                .into_iter()
                .collect::<String>();
            self.content = &self.content[1..];
            StringConstant(st)
        } else {
            Symbol(first_char)
        };

        Some(term)
    }
}
