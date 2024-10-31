use std::{cell::RefCell, rc::Rc};

use lv8_parser::{Either, Statement as StatementAST};

use super::{
    flow_control, function,
    scope::{self, Scope, ValueType},
};

pub fn run_statement(scope: &Rc<RefCell<Scope>>, statement: &StatementAST) -> ValueType {
    match statement {
        StatementAST::Assignment { left, right } => {
            let value = match right {
                Either::Left(expression) => scope::expression_to_value(scope, expression),
                Either::Right(statement) => run_statement(scope, statement),
            };

            for variables in left {
                scope.borrow_mut().set(variables, value.clone());
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
        } => flow_control::if_statement(scope, (condition, body), else_if, else_body),
        StatementAST::While { condition, body } => {
            flow_control::while_statement(scope, condition, body)
        }
    }
}
