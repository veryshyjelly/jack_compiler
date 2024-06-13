use crate::grammar::expression::{
    Expression, ExpressionList, KeywordConstant, Op, OpTerm, SubroutineCall, Term, UnaryOp,
};
use crate::grammar::statement::{
    DoStatement, IfStatement, LetStatement, ReturnStatement, Statement, Statements, WhileStatement,
};
use crate::grammar::structure::{
    Class, ClassVarDec, ClassVarType, Parameter, ParameterList, ReturnType, SubroutineBody,
    SubroutineDec, SubroutineType, Type, VarDec,
};
use crate::grammar::terminal::Terminal::{IntegerConstant, StringConstant, Symbol};
use crate::grammar::terminal::{Identifier, Keyword, Terminal};
use crate::lexer::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    pending_elements: Vec<Terminal>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            lexer: Lexer::new(content),
            pending_elements: vec![],
        }
    }

    /** 'class' className '{' classVarDec* subroutineDec* '}' */
    pub fn next_class(&mut self) -> Result<Class, String> {
        self.consume(Terminal::Keyword(Keyword::Class))?;
        let class_name = self
            .next_element()
            .ok_or("expected class name")?
            .identifier()?;

        let mut res = Class(class_name, vec![], vec![]);

        self.consume(Symbol('{'))?;
        while let Some(ele) = self.next_element() {
            let keyword = ele.keyword()?;
            if let Ok(class_var_type) = ClassVarType::from_keyword(&keyword) {
                res.1.push(self.next_class_var_dec(class_var_type)?);
            }
            if let Ok(subroutine_type) = SubroutineType::from_keyword(&keyword) {
                res.2.push(self.next_subroutine_dec(subroutine_type)?);
            }
            let closing_bracket = self.next_element().ok_or("} expected")?;
            self.pending_elements.push(closing_bracket.clone());
            if closing_bracket == Symbol('}') {
                break;
            }
        }
        self.consume(Symbol('}'))?;

        Ok(res)
    }

    /** ('static'|'field') type varName (',' varName)* ';' */
    fn next_class_var_dec(&mut self, class_var_type: ClassVarType) -> Result<ClassVarDec, String> {
        let var_type = Type::from_terminal(self.next_element().ok_or("variable type expected")?)?;
        let mut res = ClassVarDec(class_var_type, var_type, vec![]);
        loop {
            let var_name = self
                .next_element()
                .ok_or("variable identifier expected")?
                .identifier()?;
            res.2.push(var_name);
            let comma = self.next_element().ok_or(", or ; expected")?;
            if comma != Symbol(',') {
                self.pending_elements.push(comma);
                break;
            }
        }
        self.consume(Symbol(';'))?;
        Ok(res)
    }

    /** ('constructor'|'function'|'method') ('void'|type) subroutineName '(' parameterList ')' subroutineBody */
    fn next_subroutine_dec(
        &mut self,
        subroutine_type: SubroutineType,
    ) -> Result<SubroutineDec, String> {
        let return_type =
            ReturnType::from_terminal(self.next_element().ok_or("return type expected")?)?;
        let subroutine_name = self
            .next_element()
            .ok_or("subroutine name expected")?
            .identifier()?;

        self.consume(Symbol('('))?;
        let parameter_list = self.next_parameter_list()?;
        self.consume(Symbol(')'))?;

        let res = SubroutineDec(
            subroutine_type,
            return_type,
            subroutine_name,
            parameter_list,
            self.next_subroutine_body()?,
        );

        Ok(res)
    }

    /** ((type varName) (',' typeVarName)* )? */
    fn next_parameter_list(&mut self) -> Result<ParameterList, String> {
        let mut res = vec![];
        while let Some(next_element) = self.next_element() {
            // check for empty parameter list
            if Type::from_terminal(next_element.clone()).is_err() {
                self.pending_elements.push(next_element);
                return Ok(res);
            }

            let var_type =
                Type::from_terminal(next_element)?;
            let var_name = self
                .next_element()
                .ok_or("variable name expected")?
                .identifier()?;
            res.push(Parameter(var_type, var_name));
            
            let comma = self.next_element().ok_or(", or ) expected")?;
            if comma != Symbol(',') {
                self.pending_elements.push(comma);
                break;
            }
        }
        Ok(res)
    }

    /** '{' varDec* statements '}' */
    fn next_subroutine_body(&mut self) -> Result<SubroutineBody, String> {
        self.consume(Symbol('{'))?;
        let mut res = SubroutineBody(vec![], vec![]);
        while let Some(ele) = self.next_element() {
            if let Ok(next_keyword) = ele.clone().keyword() {
                match next_keyword {
                    Keyword::Var => res.0.push(self.next_var_dec()?),
                    e => res.1.push(self.next_statement(e)?),
                };
            } else {
                self.pending_elements.push(ele);
                break;
            }
        }
        self.consume(Symbol('}'))?;
        Ok(res)
    }

    /** 'var' type varName (',' varName)* ';' */
    fn next_var_dec(&mut self) -> Result<VarDec, String> {
        let typ = Type::from_terminal(self.next_element().ok_or("variable type expected")?)?;
        let mut vars = vec![];
        loop {
            let var_name = self
                .next_element()
                .ok_or("variable name expected")?
                .identifier()?;
            vars.push(var_name);
            let comma = self.next_element().ok_or(", or ; expected")?;
            if comma != Symbol(',') {
                self.pending_elements.push(comma);
                break;
            }
        }
        self.consume(Symbol(';'))?;
        Ok(VarDec(typ, vars))
    }

    /** statement* */
    fn next_statements(&mut self) -> Result<Statements, String> {
        let mut res = vec![];
        while let Some(keyword) = self.next_element() {
            if keyword.clone().keyword().is_err() {
                self.pending_elements.push(keyword);
                break;
            }
            if let Ok(statement) = self.next_statement(keyword.keyword()?) {
                res.push(statement);
            } else {
                break;
            }
        }
        Ok(res)
    }

    /** letStatement | ifStatement | whileStatement | doStatement | returnStatement */
    fn next_statement(&mut self, kind: Keyword) -> Result<Statement, String> {
        use Statement::*;
        let res = match kind {
            Keyword::Let => LetStatement(self.next_let_statement()?),
            Keyword::If => IfStatement(self.next_if_statement()?),
            Keyword::While => WhileStatement(self.next_while_statement()?),
            Keyword::Do => DoStatement(self.next_do_statement()?),
            Keyword::Return => ReturnStatement(self.next_return_statement()?),
            e => {
                self.pending_elements.push(Terminal::Keyword(kind));
                Err(format!("statement expected found: {e:?}"))?
            }
        };
        Ok(res)
    }

    /** 'let' varName ('[' expression ']')? '=' expression ';' */
    fn next_let_statement(&mut self) -> Result<LetStatement, String> {
        let var_name = self
            .next_element()
            .ok_or("variable name expected")?
            .identifier()?;
        let mut index: Option<Expression> = None;

        let opening_bracket = self.next_element().ok_or("[ or = expected")?;
        if opening_bracket == Symbol('[') {
            let _ = index.insert(self.next_expression()?);
            self.consume(Symbol(']'))?;
        } else {
            self.pending_elements.push(opening_bracket);
        }

        self.consume(Symbol('='))?;
        let expression = self.next_expression()?;
        self.consume(Symbol(';'))?;

        Ok(LetStatement(var_name, index, expression))
    }

    fn next_if_statement(&mut self) -> Result<IfStatement, String> {
        self.consume(Symbol('('))?;
        let expression = self.next_expression()?;
        self.consume(Symbol(')'))?;
        self.consume(Symbol('{'))?;
        let statements = self.next_statements()?;
        self.consume(Symbol('}'))?;
        let mut else_statement: Option<Statements> = None;
        if let Some(else_state) = self.next_element() {
            if else_state == Terminal::Keyword(Keyword::Else) {
                self.consume(Symbol('{'))?;
                let _ = else_statement.insert(self.next_statements()?);
                self.consume(Symbol('}'))?;
            } else {
                self.pending_elements.push(else_state);
            }
        }
        Ok(IfStatement(expression, statements, else_statement))
    }

    fn next_while_statement(&mut self) -> Result<WhileStatement, String> {
        self.consume(Symbol('('))?;
        let expression = self.next_expression()?;
        self.consume(Symbol(')'))?;
        self.consume(Symbol('{'))?;
        let statements = self.next_statements()?;
        self.consume(Symbol('}'))?;
        Ok(WhileStatement(expression, statements))
    }

    fn next_do_statement(&mut self) -> Result<DoStatement, String> {
        let caller_name = self
            .next_element()
            .ok_or("class or var or function name expected")?
            .identifier()?;
        let subroutine_call = self.next_subroutine_call(caller_name)?;
        self.consume(Symbol(';'))?;
        Ok(DoStatement(subroutine_call))
    }

    fn next_return_statement(&mut self) -> Result<ReturnStatement, String> {
        let next_element = self.next_element().ok_or("; expected")?;
        if next_element == Symbol(';') {
            Ok(ReturnStatement(None))
        } else {
            self.pending_elements.push(next_element);
            let expression = self.next_expression()?;
            self.consume(Symbol(';'))?;
            Ok(ReturnStatement(Some(expression)))
        }
    }

    fn next_expression_list(&mut self) -> Result<ExpressionList, String> {
        let mut res = vec![];
        while let Ok(expression) = self.next_expression() {
            res.push(expression);
            let comma = self.next_element().ok_or(", or ) expected")?;
            if comma != Symbol(',') {
                self.pending_elements.push(comma);
                break;
            }
        }
        Ok(res)
    }

    fn next_expression(&mut self) -> Result<Expression, String> {
        let term = self.next_term()?;
        let mut res = Expression(term, vec![]);

        loop {
            let next_term = self.next_element().ok_or("end of expression expected")?;
            if let Err(e) = next_term.clone().symbol() {
                self.pending_elements.push(next_term);
                return Err(e);
            }
            let op = next_term.symbol()?;
            if let Some(operation) = Op::from_char(op) {
                let next_term = self.next_term()?;
                res.1.push(OpTerm(operation, next_term));
            } else {
                self.pending_elements.push(Symbol(op));
                break;
            }
        }

        Ok(res)
    }

    fn next_term(&mut self) -> Result<Term, String> {
        let element = self.next_element().ok_or("term expected")?;

        let res = match element {
            Symbol(c) => {
                if c == '(' {
                    let expression = self.next_expression()?;
                    self.consume(Symbol(')'))?;
                    Term::BracketExpression(Box::new(expression))
                } else if let Some(un) = UnaryOp::from_char(c) {
                    let next_term = self.next_term()?;
                    Term::UnaryOpTerm(un, Box::new(next_term))
                } else {
                    self.pending_elements.push(element);
                    Err(format!("term expected found symbol: {c}"))?
                }
            }
            IntegerConstant(val) => Term::IntegerConstant(val),
            StringConstant(val) => Term::StringConstant(val),
            Terminal::Keyword(val) => Term::KeywordConstant(KeywordConstant::from_keyword(val)?),
            Terminal::Identifier(val) => {
                let square_bracket = self.next_element().ok_or("end of expression expected")?;
                if square_bracket == Symbol('[') {
                    let index = self.next_expression()?;
                    self.consume(Symbol(']'))?;
                    Term::VarNameIndex(val, Box::new(index))
                } else if let Ok(subroutine_call) = self.next_subroutine_call(val.clone()) {
                    Term::SubroutineCall(subroutine_call)
                } else {
                    Term::VarName(val)
                }
            }
        };

        Ok(res)
    }

    fn next_subroutine_call(&mut self, val: Identifier) -> Result<SubroutineCall, String> {
        let dot_or_bracket = self.next_element().ok_or("end of expression expected")?;
        let res = if dot_or_bracket == Symbol('(') {
            let expression_list = self.next_expression_list()?;
            self.consume(Symbol(')'))?;
            SubroutineCall(None, val, expression_list)
        } else if dot_or_bracket == Symbol('.') {
            let subroutine_name = self
                .next_element()
                .ok_or("subroutine name expected")?
                .identifier()?;
            self.consume(Symbol('('))?;
            let expression_list = self.next_expression_list()?;
            self.consume(Symbol(')'))?;
            SubroutineCall(Some(val), subroutine_name, expression_list)
        } else {
            self.pending_elements.push(dot_or_bracket);
            Err("expected subroutine call")?
        };
        Ok(res)
    }

    fn consume(&mut self, term: Terminal) -> Result<(), String> {
        let next_term = self
            .next_element()
            .ok_or(format!("expected {:?} found EOF", term))?;
        if next_term != term {
            self.pending_elements.push(next_term.clone());
            Err(format!("expected {:?} found {:?}", term, next_term))?
        }
        Ok(())
    }

    fn next_element(&mut self) -> Option<Terminal> {
        if self.pending_elements.is_empty() {
            return self.lexer.next_element();
        }
        self.pending_elements.pop()
    }
}
