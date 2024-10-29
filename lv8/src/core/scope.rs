use std::collections::HashMap;
use std::fmt;

use lv8_parser::Expression as ExpressionAST;

use super::{expression::Expression, function::Function, PrimitiveTypes};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ValueType {
    Function(Function),
    Variable(PrimitiveTypes),
    InternalFunction(fn(Vec<ValueType>) -> ValueType),
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Function(function) => write!(f, "{}", function),
            ValueType::Variable(value) => write!(f, "{}", value),
            ValueType::InternalFunction(func) => {
                write!(f, "<<internal function {:?}>>", &func as *const _)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub variables: HashMap<String, ValueType>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, variable: ValueType) {
        self.variables.insert(name, variable);
    }

    pub fn get_variable(&self, name: &str) -> Option<&ValueType> {
        self.variables.get(name)
    }

    pub fn extend_variables(&mut self, variables: HashMap<String, ValueType>) {
        self.variables.extend(variables);
    }
}

pub fn expression_to_value(scope: &Scope, expression: &ExpressionAST) -> ValueType {
    let value = match expression {
        ExpressionAST::Identifier(ref identifier) => scope.get_variable(identifier).cloned(),
        _ => Some(ValueType::Variable(Expression::parse_expression(
            scope,
            expression.clone(),
        ))),
    };

    if let Some(value) = value {
        value
    } else {
        panic!("Variable not found: {:?}", expression);
    }
}
