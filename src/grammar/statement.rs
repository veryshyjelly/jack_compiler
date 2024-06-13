use crate::grammar::expression::{Expression, SubroutineCall};
use crate::grammar::structure::VarName;
use serde::Serialize;

pub type Statements = Vec<Statement>;
#[derive(Debug, Serialize)]
pub enum Statement {
    LetStatement(LetStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    DoStatement(DoStatement),
    ReturnStatement(ReturnStatement),
}
#[derive(Debug, Serialize)]
pub struct LetStatement(pub VarName, pub Option<Index>, pub Expression);
type Index = Expression;
#[derive(Debug, Serialize)]
pub struct IfStatement(pub Expression, pub Statements, pub Option<ElseStatement>);
pub type ElseStatement = Statements;
#[derive(Debug, Serialize)]
pub struct WhileStatement(pub Expression, pub Statements);
#[derive(Debug, Serialize)]
pub struct DoStatement(pub SubroutineCall);
#[derive(Debug, Serialize)]
pub struct ReturnStatement(pub Option<Expression>);
