use crate::grammar::expression::Expression;
use crate::grammar::statement::{IfStatement, LetStatement, ReturnStatement, WhileStatement};
use crate::grammar::structure::{Class, ClassVarDec, ClassVarType, Parameter, ReturnType, SubroutineBody, SubroutineDec, SubroutineType, Type, VarDec};
use crate::grammar::terminal::Terminal::Symbol;
use crate::grammar::terminal::{Keyword, Terminal};
use crate::lexer::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            lexer: Lexer::new(content),
        }
    }

    pub fn next_class(&mut self) -> Result<Class, String> {
        self.consume(Terminal::Keyword(Keyword::Class))?;
        let class_name = self
            .next_element()
            .ok_or("expected class name")?
            .identifier()
            .ok_or("expected class name")?;

        let mut res = Class(class_name, vec![], vec![]);

        self.consume(Terminal::Symbol('{'))?;
        while let Some(ele) = self.next_element() {
            if ele == Terminal::Symbol('}') {
                break;
            }
            let keyword = ele
                .keyword()
                .ok_or("expected variable declaration or subroutine declaration")?;
            if let Some(class_var_type) = ClassVarType::from_keyword(&keyword) {
                res.1.push(self.next_class_var_dec(class_var_type)?);
            }
            if let Some(subroutine_type) = SubroutineType::from_keyword(&keyword) {
                res.2.push(self.next_subroutine_dec(subroutine_type)?);
            }
        }

        Ok(res)
    }

    fn next_class_var_dec(&mut self, class_var_type: ClassVarType) -> Result<ClassVarDec, String> {
        let var_type = Type::from_terminal(self.next_element().ok_or("variable type expected")?)
            .ok_or("variable type expected")?;
        let mut res = ClassVarDec(class_var_type, var_type, vec![]);
        while let Some(ele) = self.next_element() {
            if ele == Symbol(';') {
                break;
            } else if ele == Symbol(',') {
                continue;
            }
            res.2
                .push(ele.identifier().ok_or("variable name expected")?);
        }
        Ok(res)
    }

    fn next_subroutine_dec(
        &mut self,
        subroutine_type: SubroutineType,
    ) -> Result<SubroutineDec, String> {
        let return_type =
            ReturnType::from_terminal(self.next_element().ok_or("return type expected")?)
                .ok_or("return type expected")?;
        let subroutine_name = self
            .next_element()
            .ok_or("subroutine name expected")?
            .identifier()
            .ok_or("subroutine name expected")?;
        self.consume(Symbol('('))?;
        let mut params = vec![];
        while let Some(ele) = self.next_element() {
            if ele == Symbol(')') {
                break;
            } else if ele == Symbol(',') {
                continue;
            }
            params.push(self.next_parameter()?);
        }
        Ok(SubroutineDec(
            subroutine_type,
            return_type,
            subroutine_name,
            params,
            self.next_subroutine_body()?,
        ))
    }

    fn next_parameter(&mut self) -> Result<Parameter, String> {
        let var_type = Type::from_terminal(self.next_element().ok_or("parameter type expected")?)
            .ok_or("parameter type expected")?;
        let var_name = self
            .next_element()
            .ok_or("variable name expected")?
            .identifier()
            .ok_or("variable name expected")?;
        Ok(Parameter(var_type, var_name))
    }

    fn next_subroutine_body(&mut self) -> Result<SubroutineBody, String> {
        self.consume(Symbol('{'))?;
        panic!("not implemented")
    }

    fn next_var_dec(&mut self) -> Result<VarDec, String> {
        panic!("not implemented")
    }

    fn next_let_statement(&mut self) -> Result<LetStatement, String> {
        panic!("not implemented")
    }

    fn next_if_statement(&mut self) -> Result<IfStatement, String> {
        panic!("not implemented")
    }

    fn next_while_statement(&mut self) -> Result<WhileStatement, String> {
        panic!("not implemented")
    }

    fn next_return_statement(&mut self) -> Result<ReturnStatement, String> {
        panic!("not implemented")
    }

    fn next_expression(&mut self) -> Result<Expression, String> {
        panic!("not implemented")
    }

    fn next_subroutine_call(&mut self) -> Result<Expression, String> {
        panic!("not implemented")
    }

    fn consume(&mut self, term: Terminal) -> Result<(), String> {
        let next_term = self
            .lexer
            .next_element()
            .ok_or(format!("expected {:?} found EOF", term))?;
        if next_term != term {
            Err(format!("expected {:?} found {:?}", term, next_term))?
        }
        Ok(())
    }

    fn next_element(&mut self) -> Option<Terminal> {
        return self.lexer.next_element();
    }
}
