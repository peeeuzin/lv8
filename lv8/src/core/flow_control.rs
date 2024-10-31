use std::{cell::RefCell, rc::Rc};

use super::{
    block::Block,
    expression::value_to_bool,
    scope::{self, Scope, ValueType},
    PrimitiveTypes,
};
use lv8_parser::{Block as BlockAST, Expression as ExpressionAST};

pub fn if_statement(
    scope: &Rc<RefCell<Scope>>,
    r#if: (&ExpressionAST, &BlockAST),
    else_if: &[(ExpressionAST, BlockAST)],
    else_body: &Option<BlockAST>,
) -> ValueType {
    let condition = scope::expression_to_value(scope, r#if.0);

    // if the condition is true, execute the body
    if value_to_bool(condition) {
        let scope = Scope::with_parent("if", scope.clone());

        return Block::new(r#if.1.clone(), Rc::new(RefCell::new(scope))).call();
    }

    // if the condition is false, check the else if conditions
    for (condition, block) in else_if {
        let condition = scope::expression_to_value(scope, condition);

        if value_to_bool(condition) {
            let scope = Scope::with_parent("ifelse", scope.clone());

            return Block::new(block.clone(), Rc::new(RefCell::new(scope))).call();
        }
    }

    // if no condition is true, execute the else body if it exists
    if let Some(else_body) = else_body {
        let scope = Scope::with_parent("else", scope.clone());

        return Block::new(else_body.clone(), Rc::new(RefCell::new(scope))).call();
    }

    // if no condition is true, return undefined
    ValueType::Variable(PrimitiveTypes::Undefined)
}

pub fn while_statement(
    scope: &Rc<RefCell<Scope>>,
    condition: &ExpressionAST,
    body: &BlockAST,
) -> ValueType {
    let mut return_value = ValueType::Variable(PrimitiveTypes::Undefined);

    while value_to_bool(scope::expression_to_value(scope, condition)) {
        let scope = Scope::with_parent("while", scope.clone());

        return_value = Block::new(body.clone(), Rc::new(RefCell::new(scope))).call();
    }

    return_value
}