use lv8_parser::{Either, Statement as StatementAST};

use super::{
    function,
    scope::{self, Scope, ValueType},
};

pub fn run_statement(scope: &mut Scope, statement: &StatementAST) -> ValueType {
    match statement {
        StatementAST::Assignment { left, right } => {
            let value = match right {
                Either::Left(expression) => scope::expression_to_value(scope, expression),
                Either::Right(statement) => run_statement(scope, statement),
            };

            for variables in left {
                scope.set_variable(variables.to_string(), value.clone());
            }

            value
        }
        StatementAST::FunctionDefinition {
            name,
            parameters,
            body,
        } => function::handle_function_definition(scope, name, parameters, body),
        StatementAST::FunctionCall { name, arguments } => {
            function::handle_function_call(scope, name, arguments)
        }
    }
}
