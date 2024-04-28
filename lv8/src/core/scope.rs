use std::collections::HashMap;

use lv8_parser::Expression as ExpressionAST;

use super::{expression::Expression, function::Function, PrimitiveTypes};

#[derive(Clone, Debug)]
pub enum ValueType {
    Function(Function),
    Variable(PrimitiveTypes),
    InternalFunction(fn(Vec<ValueType>) -> ValueType),
}

impl ToString for ValueType {
    fn to_string(&self) -> String {
        match self {
            ValueType::Function(function) => function.to_string(),
            ValueType::Variable(value) => value.to_string(),
            ValueType::InternalFunction(f) => {
                format!("<<internal function {:?}>>", &f as *const _).to_string()
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
