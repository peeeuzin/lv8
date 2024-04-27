use std::collections::HashMap;

use lv8_parser::Expression as ExpressionAST;

use super::{expression::Expression, function::Function, PrimitiveTypes};

#[derive(Clone, Debug)]
pub enum VariableType {
    Function(Function),
    Variable(PrimitiveTypes),
    InternalFunction(fn(Vec<VariableType>) -> VariableType),
}

impl ToString for VariableType {
    fn to_string(&self) -> String {
        match self {
            VariableType::Function(function) => function.to_string(),
            VariableType::Variable(value) => value.to_string(),
            VariableType::InternalFunction(f) => {
                format!("<<internal function {:?}>>", &f as *const _).to_string()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub variables: HashMap<String, VariableType>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, variable: VariableType) {
        self.variables.insert(name, variable);
    }

    pub fn get_variable(&self, name: &str) -> Option<&VariableType> {
        self.variables.get(name)
    }

    pub fn extend_variables(&mut self, variables: HashMap<String, VariableType>) {
        self.variables.extend(variables);
    }
}

pub fn expression_to_value(scope: &Scope, expression: &ExpressionAST) -> VariableType {
    let value = match expression {
        ExpressionAST::Identifier(ref identifier) => scope.get_variable(identifier).cloned(),
        _ => Some(VariableType::Variable(Expression::parse_expression(
            expression.clone(),
        ))),
    };

    if let Some(value) = value {
        value
    } else {
        panic!("Variable not found: {:?}", expression);
    }
}
