use std::{cell::RefCell, rc::Rc};

use lv8_common::error::Result;
use lv8_parser::{Either, Statement as StatementAST};

use super::{
    flow_control, function, import,
    module::Module,
    scope::{self, Scope, ValueType},
    Metadata,
};

pub fn run_statement(
    scope: &Rc<RefCell<Scope>>,
    statement: &StatementAST,
    metadata: &Rc<Metadata>,
) -> Result<ValueType> {
    match statement {
        StatementAST::Assignment { left, right } => {
            let value = match right {
                Either::Left(expression) => scope::evaluate_expression(scope, expression)?,
                Either::Right(statement) => run_statement(scope, statement, metadata)?,
            };

            for variables in left {
                scope.borrow_mut().set(variables, value.clone());
            }

            Ok(value)
        }
        StatementAST::FunctionDefinition {
            name,
            parameters,
            body,
        } => Ok(function::handle_function_definition(
            scope, name, parameters, body, metadata,
        )),
        StatementAST::FunctionCall {
            expression,
            arguments,
        } => function::handle_function_call(scope, expression, arguments, metadata),
        StatementAST::If {
            condition,
            body,
            else_if,
            else_body,
        } => Ok(flow_control::if_statement(
            scope,
            (condition, body),
            else_if,
            else_body,
            metadata,
        )?),
        StatementAST::While { condition, body } => Ok(flow_control::while_statement(
            scope, condition, body, metadata,
        )?),
        StatementAST::ModuleDefinition { name, body } => {
            let module = Module::new(name, Rc::clone(scope), body, metadata)?;

            scope.borrow_mut().set(name, ValueType::Module(module));

            Ok(ValueType::Variable(super::PrimitiveTypes::Undefined))
        }
        StatementAST::Import { path, ident } => {
            Ok(import::import_statement(scope, path, ident, metadata)?)
        }
    }
}
