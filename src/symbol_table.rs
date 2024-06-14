use crate::grammar::structure::Type;
use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, Variable>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
    pub fn reset(&mut self) {
        self.table.clear();
    }

    pub fn define(&mut self, name: String, variable_type: Type, kind: Kind) -> bool {
        let index = self.var_count(kind);
        self.table
            .insert(
                name,
                Variable {
                    variable_type,
                    kind,
                    index,
                },
            )
            .is_some()
    }

    pub fn var_count(&self, kind: Kind) -> usize {
        self.table.values().filter(|x| x.kind == kind).count()
    }

    pub fn kind_of(&self, name: &String) -> Option<Kind> {
        Some(self.table.get(name)?.kind)
    }

    pub fn type_of(&self, name: &String) -> Option<Type> {
        Some(self.table.get(name)?.variable_type.clone())
    }

    pub fn index_of(&self, name: &String) -> Option<usize> {
        Some(self.table.get(name)?.index)
    }

    pub fn mapping_of(&self, name: &String) -> Result<String, String> {
        let kind = self
            .kind_of(name)
            .ok_or(format!("undefined variable: {name}"))?;
        let index = self
            .index_of(name)
            .ok_or(format!("undefined variable: {name}"))?;
        let res = match kind {
            Kind::Field => format!("this {index}"),
            Kind::Static => format!("static {index}"),
            Kind::Arg => format!("argument {index}"),
            Kind::Var => format!("local {index}"),
        };
        Ok(res)
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct Variable {
    variable_type: Type,
    kind: Kind,
    index: usize,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Kind {
    Field,
    Static,
    Arg,
    Var,
}
