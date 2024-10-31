use owo_colors::OwoColorize;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};

use lv8_parser::Expression as ExpressionAST;

use super::{expression::Expression, function::Function, PrimitiveTypes};

#[derive(Clone, PartialEq, PartialOrd)]
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

impl Debug for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Function(function) => write!(f, "{:?}", function.bright_magenta()),
            ValueType::Variable(value) => write!(f, "{:?}", value),
            ValueType::InternalFunction(func) => {
                write!(
                    f,
                    "{}",
                    format!("<<internal function {:?}>>", &func as *const _).bright_magenta()
                )
            }
        }
    }
}

#[derive(Clone)]
pub struct Scope {
    pub parent: Option<Rc<RefCell<Scope>>>,
    pub variables: HashMap<String, ValueType>,
    pub name: String,
}

impl Scope {
    pub fn new(name: &str) -> Self {
        Self {
            parent: None,
            name: name.to_owned(),
            variables: HashMap::new(),
        }
    }

    pub fn with_parent(name: &str, parent: Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(parent),
            name: name.to_owned(),
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: ValueType) {
        match self.variables.get_mut(name) {
            Some(variable) => *variable = value,
            None => match &mut self.parent {
                Some(parent) => parent.borrow_mut().set(name, value),
                None => {
                    self.variables.insert(name.to_owned(), value);
                }
            },
        }
    }

    pub fn get(&self, name: &str) -> Option<ValueType> {
        match self.variables.get(name) {
            Some(value) => Some(value.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn extend(&mut self, variables: HashMap<String, ValueType>) {
        self.variables.extend(variables);
    }
}

pub fn expression_to_value(scope: &Rc<RefCell<Scope>>, expression: &ExpressionAST) -> ValueType {
    let value = match expression {
        ExpressionAST::Identifier(ref identifier) => scope.borrow().get(identifier),
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

impl Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parent = match &self.parent {
            Some(parent) => parent.borrow().name.to_string(),
            None => "None".to_string(),
        };

        write!(
            f,
            "<<scope {}>> <<parent {}>>: {:#?}",
            self.name, parent, self.variables
        )
    }
}
