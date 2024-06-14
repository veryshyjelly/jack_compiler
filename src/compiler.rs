use crate::grammar::expression::{Expression, KeywordConstant, Op, SubroutineCall, Term, UnaryOp};
use crate::grammar::statement::{Statement, Statements};
use crate::grammar::structure::{
    Class, ClassName, ClassVarType, SubroutineDec, SubroutineName, SubroutineType, Type,
};
use crate::grammar::terminal::Identifier;
use crate::symbol_table::{Kind, SymbolTable};
use std::ops::Deref;

pub struct Compiler {
    /**the name of the currently compiling class*/
    class_name: ClassName,
    /**name of the currently compiling subroutine*/
    subroutine_name: SubroutineName,
    label_count: usize,
    class_symbol_table: SymbolTable,
    subroutine_symbol_table: SymbolTable,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            class_name: Identifier("".into()),
            subroutine_name: Identifier("".into()),
            label_count: 0,
            class_symbol_table: SymbolTable::new(),
            subroutine_symbol_table: SymbolTable::new(),
        }
    }

    pub fn compile_class(&mut self, class: Class) -> Result<Vec<String>, String> {
        self.class_name = class.0;

        self.class_symbol_table.reset();
        // Create the symbol table for class
        for var_dec in class.1 {
            let kind = match var_dec.0 {
                ClassVarType::Static => Kind::Static,
                ClassVarType::Field => Kind::Field,
            };
            var_dec.2.iter().for_each(|var| {
                self.class_symbol_table
                    .define(var.0.clone(), var_dec.1.clone(), kind);
            });
        }
        // Compile all the subroutines and append them into the result
        let mut res = vec![];
        for sub_routine_dec in class.2 {
            let mut vm_commands = match sub_routine_dec.0 {
                SubroutineType::Constructor => self.compile_constructor(sub_routine_dec)?,
                SubroutineType::Function => self.compile_function(sub_routine_dec)?,
                SubroutineType::Method => self.compile_method(sub_routine_dec)?,
            };
            res.append(&mut vm_commands)
        }
        Ok(res)
    }

    fn compile_constructor(
        &mut self,
        subroutine_dec: SubroutineDec,
    ) -> Result<Vec<String>, String> {
        let statements = self.create_subroutine_symbol_table(subroutine_dec)?;
        // Write the initial code for constructor
        let mut res = vec![
            format!(
                "function {}.{} {}",
                self.class_name.0,
                self.subroutine_name.0,
                self.subroutine_symbol_table.var_count(Kind::Var)
            ),
            format!(
                "push constant {}",
                self.class_symbol_table.var_count(Kind::Field)
            ),
            "call Memory.alloc 1".into(),
            "pop pointer 0".into(),
        ];
        // Compile all the statements
        res.append(
            &mut self
                .compile_statements(statements)?
                .into_iter()
                .map(|x| format!("  {x}"))
                .collect(),
        );
        Ok(res)
    }

    fn compile_method(&mut self, subroutine_dec: SubroutineDec) -> Result<Vec<String>, String> {
        let statements = self.create_subroutine_symbol_table(subroutine_dec)?;
        // Write the initial code for method call
        let mut res = vec![
            format!(
                "function {}.{} {}",
                self.class_name.0,
                self.subroutine_name.0,
                self.subroutine_symbol_table.var_count(Kind::Var)
            ),
            "push argument 0".into(),
            "pop pointer 0".into(),
        ];
        // Compile all the statements
        res.append(
            &mut self
                .compile_statements(statements)?
                .into_iter()
                .map(|x| format!("  {x}"))
                .collect(),
        );
        Ok(res)
    }

    fn compile_function(&mut self, subroutine_dec: SubroutineDec) -> Result<Vec<String>, String> {
        let statements = self.create_subroutine_symbol_table(subroutine_dec)?;
        // Write the initial code for method call
        let mut res = vec![format!(
            "function {}.{} {}",
            self.class_name.0,
            self.subroutine_name.0,
            self.subroutine_symbol_table.var_count(Kind::Var)
        )];
        // Compile all the statements
        res.append(
            &mut self
                .compile_statements(statements)?
                .into_iter()
                .map(|x| format!("  {x}"))
                .collect(),
        );
        Ok(res)
    }

    fn compile_statements(&mut self, statements: Statements) -> Result<Vec<String>, String> {
        let mut res = vec![];
        // Compile all the statements and push them in the result
        for statement in statements {
            res.append(&mut self.compile_statement(statement)?);
        }
        Ok(res)
    }

    fn compile_statement(&mut self, statement: Statement) -> Result<Vec<String>, String> {
        let mut res = vec![];
        match statement {
            Statement::LetStatement(s) => {
                if let Some(idx) = s.1 {
                    // Array indexing done
                    res.push(format!("push {}", self.mapping_of(&s.0 .0)?));
                    res.append(&mut self.compile_expression(&idx)?);
                    res.push("add".into());
                    res.append(&mut self.compile_expression(&s.2)?);
                    res.extend([
                        "pop temp 0".into(),
                        "pop pointer 1".into(),
                        "push temp 0".into(),
                        "pop that 0".into(),
                    ]);
                } else {
                    res.append(&mut self.compile_expression(&s.2)?);
                    res.push(format!("pop {}", self.mapping_of(&s.0 .0)?));
                }
            }
            Statement::IfStatement(s) => {
                self.label_count += 1;
                // Compute the if condition
                res.append(&mut self.compile_expression(&s.0)?);
                res.push("not".into());

                let if_label = format!("IF_LABEL${}", self.label_count);
                let else_label = format!("ELSE_LABEL${}", self.label_count);

                res.push(format!("if-goto {}", else_label));
                // Compile the statements in the if block
                res.append(
                    &mut self
                        .compile_statements(s.1)?
                        .into_iter()
                        .map(|x| format!("  {x}"))
                        .collect(),
                );
                res.push(format!("goto {}", if_label));
                res.push(format!("label {}", else_label));
                // Compile the statements in the else block
                if let Some(else_statements) = s.2 {
                    res.append(
                        &mut self
                            .compile_statements(else_statements)?
                            .into_iter()
                            .map(|x| format!("  {x}"))
                            .collect(),
                    );
                }
                res.push(format!("label {}", if_label));
            }
            Statement::WhileStatement(s) => {
                self.label_count += 1;
                let while_label = format!("WHILE_LABEL${}", self.label_count);
                let break_label = format!("BREAK_LABEL${}", self.label_count);

                res.push(format!("label {}", while_label));
                // Compute the while condition
                res.append(&mut self.compile_expression(&s.0)?);
                res.push("not".into());
                res.push(format!("if-goto {}", break_label));
                // Compile the statements in the while block
                res.append(
                    &mut self
                        .compile_statements(s.1)?
                        .into_iter()
                        .map(|x| format!("  {x}"))
                        .collect(),
                );
                res.push(format!("goto {}", while_label));
                res.push(format!("label {}", break_label));
            }
            Statement::DoStatement(s) => {
                res.append(&mut self.compile_subroutine_call(&s.0)?);
            }
            Statement::ReturnStatement(s) => {
                if let Some(exp) = s.0 {
                    res.append(&mut self.compile_expression(&exp)?);
                    res.push("return".into())
                }
            }
        }
        Ok(res)
    }

    fn compile_subroutine_call(&self, sub_call: &SubroutineCall) -> Result<Vec<String>, String> {
        let mut res = vec![];
        if let Some(name) = sub_call.0.as_ref() {
            if let Ok(mapping) = self.mapping_of(&name.0) {
                // Method call
                res.push(format!("push {}", mapping));
                for exp in sub_call.2.iter() {
                    res.append(&mut self.compile_expression(exp)?);
                }
                res.push(format!(
                    "call {}.{} {}",
                    self.class_name_of(&name.0)?,
                    sub_call.1 .0,
                    sub_call.2.len() + 1
                ));
            } else {
                // Function call to another class
                for exp in sub_call.2.iter() {
                    res.append(&mut self.compile_expression(exp)?);
                }
                res.push(format!(
                    "call {}.{} {}",
                    name.0,
                    sub_call.1 .0,
                    sub_call.2.len()
                ));
            }
        } else {
            // Method call to this class static functions cannot be called like this
            res.push("push pointer 0".into());
            for exp in sub_call.2.iter() {
                res.append(&mut self.compile_expression(exp)?);
            }
            res.push(format!(
                "call {}.{} {}",
                self.class_name.0,
                sub_call.1 .0,
                sub_call.2.len() + 1
            ));
        }
        Ok(res)
    }

    fn compile_expression(&self, expression: &Expression) -> Result<Vec<String>, String> {
        let mut res = vec![];
        res.append(&mut self.compile_term(&expression.0)?);
        for op_term in &expression.1 {
            res.append(&mut self.compile_term(&op_term.1)?);
            res.push(
                match op_term.0 {
                    Op::Add => "add",
                    Op::Sub => "sub",
                    Op::Mul => "call Math.multiply 2",
                    Op::Div => "call Math.divide 2",
                    Op::And => "and",
                    Op::Or => "or",
                    Op::Lt => "lt",
                    Op::Gt => "gt",
                    Op::Eq => "eq",
                }
                .into(),
            );
        }
        Ok(res)
    }

    fn compile_term(&self, term: &Term) -> Result<Vec<String>, String> {
        let mut res = vec![];
        match term {
            Term::IntegerConstant(c) => res.push(format!("push constant {c}")),
            Term::StringConstant(s) => {
                res.extend([
                    format!("push constant {}", s.len()),
                    "call String.new 1".into(),
                ]);
                for c in s.chars() {
                    res.push(format!("push constant {}", c as u8));
                    res.push("call String.appendChar 2".into());
                }
            }
            Term::KeywordConstant(c) => {
                match c {
                    KeywordConstant::True => res.extend(["push constant 1".into(), "neg".into()]),
                    KeywordConstant::False | KeywordConstant::Null => {
                        res.extend(["push constant 0".into()])
                    }
                    KeywordConstant::This => res.extend(["push pointer 0".into()]),
                };
            }
            Term::VarName(var) => res.push(format!("push {}", self.mapping_of(&var.0)?)),
            Term::VarNameIndex(var, index) => {
                res.push(format!("push {}", self.mapping_of(&var.0)?));
                // Push the index expression
                res.append(&mut self.compile_expression(index.deref())?);
                res.push("add".into());
                // Now point to this computed address
                res.push("pop pointer 1".into());
                // Push the value at that address
                res.push("push that 0".into());
            }
            Term::BracketExpression(exp) => {
                res.append(&mut self.compile_expression(exp.deref())?);
            }
            Term::UnaryOpTerm(op, term) => {
                res.append(&mut self.compile_term(term.deref())?);
                match op {
                    UnaryOp::Minus => res.push("neg".into()),
                    UnaryOp::Not => res.push("not".into()),
                };
            }
            Term::SubroutineCall(sub) => res.append(&mut self.compile_subroutine_call(sub)?),
        }
        Ok(res)
    }

    fn create_subroutine_symbol_table(
        &mut self,
        subroutine_dec: SubroutineDec,
    ) -> Result<Statements, String> {
        // Set the subroutine name for the current subroutine
        self.subroutine_name = subroutine_dec.2;
        // Reset the table to remove any previous local variables
        self.subroutine_symbol_table.reset();
        // Create this entry in the symbol table if the subroutine is a method
        match subroutine_dec.0 {
            SubroutineType::Method => {
                self.subroutine_symbol_table.define(
                    "this".into(),
                    Type::ClassName(self.class_name.clone()),
                    Kind::Arg,
                );
            }
            _ => {}
        }
        // Create the arguments in the symbol table
        for params in subroutine_dec.3 {
            self.subroutine_symbol_table
                .define(params.1 .0, params.0, Kind::Arg);
        }
        let subroutine_body = subroutine_dec.4;
        // Create the local variables in the symbol table
        for var_dec in subroutine_body.0 {
            var_dec.1.into_iter().for_each(|var_name| {
                self.subroutine_symbol_table
                    .define(var_name.0, var_dec.0.clone(), Kind::Var);
            });
        }

        Ok(subroutine_body.1)
    }

    fn mapping_of(&self, name: &String) -> Result<String, String> {
        if let Ok(map) = self.subroutine_symbol_table.mapping_of(name) {
            return Ok(map);
        }
        self.class_symbol_table.mapping_of(name)
    }

    fn class_name_of(&self, name: &String) -> Result<String, String> {
        if let Some(tp) = self.subroutine_symbol_table.type_of(name) {
            if let Type::ClassName(Identifier(v)) = tp {
                return Ok(v);
            }
        }
        if let Some(tp) = self.class_symbol_table.type_of(name) {
            if let Type::ClassName(Identifier(v)) = tp {
                return Ok(v);
            }
        }
        Err("not a class")?
    }
}
