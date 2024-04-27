use lv8_parser::{Either, Statement as StatementAST};

use super::{
    function,
    scope::{self, Scope, VariableType},
};

pub fn run_statement(scope: &mut Scope, statement: &StatementAST) -> VariableType {
    match statement {
        StatementAST::Assignment { left, right } => {
            let value = match right {
                Either::Left(expression) => scope::expression_to_value(scope, expression),
                Either::Right(statement) => run_statement(scope, statement),
            };

            for namespace in left {
                let name = namespace.0.join(".");

                scope.set_variable(name, value.clone());
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
