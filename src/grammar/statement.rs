use crate::grammar::expression::{Expression, SubroutineCall};
use crate::grammar::structure::VarName;

pub type Statements = Vec<Statement>;
#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    DoStatement(DoStatement),
    ReturnStatement(ReturnStatement),
}
#[derive(Debug)]
pub struct LetStatement(pub VarName, pub Option<Index>, pub Expression);
type Index = Expression;
#[derive(Debug)]
pub struct IfStatement(pub Expression, pub Statements, pub Option<ElseStatement>);
pub type ElseStatement = Statements;
#[derive(Debug)]
pub struct WhileStatement(pub Expression, pub Statements);
#[derive(Debug)]
pub struct DoStatement(pub SubroutineCall);
#[derive(Debug)]
pub struct ReturnStatement(pub Option<Expression>);
