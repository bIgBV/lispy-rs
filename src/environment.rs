use syntax::ast::Expr;

use std::collections::HashMap;

pub struct Env {
    pub table: HashMap<String, Expr>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            table: HashMap::new(),
        }
    }
}
