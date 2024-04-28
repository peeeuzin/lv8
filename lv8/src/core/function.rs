use lv8_parser::{Block as BlockAST, Either, Expression as ExpressionAST, Statement};

use super::{
    block::Block,
    scope::{self, Scope, ValueType},
    PrimitiveTypes,
};

#[derive(Clone, Debug)]
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

            scope.set_variable(expected_parameter.clone(), parameter);
        }

        self.body.call()
    }
}

impl ToString for Function {
    fn to_string(&self) -> String {
        format!("<<function {}>>", self.name)
    }
}

pub fn handle_function_call(
    scope: &mut Scope,
    function_name: &str,
    arguments: &[Either<ExpressionAST, Statement>],
) -> ValueType {
    let function = scope.get_variable(function_name).cloned();

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
    scope: &mut Scope,
    name: &str,
    parameters: &[String],
    body: &BlockAST,
) -> ValueType {
    let new_function_scope = scope.clone();

    let body = Block::new(body.clone(), new_function_scope);

    let function = Function::new(name.to_string(), body, parameters.to_vec());
    scope.set_variable(name.to_string(), scope::ValueType::Function(function));

    scope::ValueType::Variable(PrimitiveTypes::Undefined)
}
