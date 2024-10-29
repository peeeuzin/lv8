use lv8_parser::{Either, Statement as StatementAST};

use super::{
    block::Block,
    expression::value_to_bool,
    function,
    scope::{self, Scope, ValueType},
    PrimitiveTypes,
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
        StatementAST::If {
            condition,
            body,
            else_if,
            else_body,
        } => {
            let condition = scope::expression_to_value(scope, condition);

            if value_to_bool(condition) {
                Block::new(body.clone(), scope.clone()).call()
            } else {
                for (condition, block) in else_if {
                    let condition = scope::expression_to_value(scope, condition);

                    if value_to_bool(condition) {
                        return Block::new(block.clone(), scope.clone()).call();
                    }
                }

                if let Some(else_body) = else_body {
                    Block::new(else_body.clone(), scope.clone()).call()
                } else {
                    ValueType::Variable(PrimitiveTypes::Undefined)
                }
            }
        }
    }
}
