use lv8_parser::{Block as BlockAST, Either, Expression as ExpressionAST, Statement};
use std::{
    cell::RefCell,
    fmt::{self, Debug},
    rc::Rc,
};

use super::{
    block::Block,
    scope::{self, Scope, ValueType},
    PrimitiveTypes,
};

#[derive(Clone)]
pub struct Function {
    name: String,
    body: Block,
    expected_parameters: Vec<String>,
}

impl Function {
    pub fn new(name: String, body: Block, expected_parameters: Vec<String>) -> Self {
        Self {
            name,
            body,
            expected_parameters,
        }
    }

    pub fn call(mut self, parameters: Vec<ValueType>) -> ValueType {
        let scope = &mut self.body.scope;

        let mut parameters = parameters.into_iter();

        for expected_parameter in &self.expected_parameters {
            let parameter = parameters
                .next()
                .unwrap_or(ValueType::Variable(PrimitiveTypes::Undefined));

            scope.borrow_mut().set(expected_parameter, parameter);
        }

        self.body.call()
    }
}

pub fn handle_function_call(
    scope: &Rc<RefCell<Scope>>,
    function_name: &str,
    arguments: &[Either<ExpressionAST, Statement>],
) -> ValueType {
    let function = scope.borrow().get(function_name);

    let args = arguments
        .iter()
        .map(|x| match x {
            Either::Left(expression) => scope::expression_to_value(scope, expression),
            Either::Right(statement) => super::statement::run_statement(scope, statement),
        })
        .collect::<Vec<scope::ValueType>>();

    if function.is_none() {
        panic!("Function not found: {}", function_name);
    }

    match function.unwrap() {
        ValueType::Function(function) => function.call(args),
        ValueType::InternalFunction(function) => function(args),
        _ => {
            panic!("{} is not a function", function_name);
        }
    }
}

pub fn handle_function_definition(
    scope: &Rc<RefCell<Scope>>,
    name: &str,
    parameters: &[String],
    body: &BlockAST,
) -> ValueType {
    let new_function_scope = Scope::with_parent(&format!("func_{}", name), scope.clone());
    let body = Block::new(body.clone(), Rc::new(RefCell::new(new_function_scope)));

    let function = Function::new(name.to_string(), body, parameters.to_vec());
    scope
        .borrow_mut()
        .set(name, scope::ValueType::Function(function));

    scope::ValueType::Variable(PrimitiveTypes::Undefined)
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<<function {}>>", self.name)
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<<function {}>>", self.name)
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Function {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}
