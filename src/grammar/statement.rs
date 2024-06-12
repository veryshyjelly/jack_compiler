use crate::grammar::expression::{Expression, SubroutineCall};
use crate::grammar::structure::VarName;

pub type Statements = Vec<Statement>;
pub enum Statement {
    LetStatement(LetStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    DoStatement(DoStatement),
    ReturnStatement(ReturnStatement),
}
pub struct LetStatement(pub VarName, pub Option<Index>, pub Expression);
type Index = Expression;
pub struct IfStatement(pub Expression, pub Statements, pub Option<ElseStatements>);
type ElseStatements = Statements;
pub struct WhileStatement(pub Expression, pub Statements);
pub struct DoStatement(SubroutineCall);
pub struct ReturnStatement(Option<Expression>);
