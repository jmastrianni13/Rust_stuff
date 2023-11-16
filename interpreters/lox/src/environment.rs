use std::collections::HashMap;
use crate::expr;

pub struct Environment {
    values: HashMap<String, expr::LiteralValue>,
}

