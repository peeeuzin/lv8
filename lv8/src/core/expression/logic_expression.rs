use std::{cell::RefCell, ops, rc::Rc};

use lv8_common::error::Result;
use lv8_parser::LogicExpression as LogicExpressionAST;

use crate::core::{
    scope::{evaluate_expression, Scope, ValueType},
    PrimitiveTypes,
};

pub fn evaluate_logic_expression(
    scope: &Rc<RefCell<Scope>>,
    logic_expression: LogicExpressionAST,
) -> Result<bool> {
    match logic_expression {
        LogicExpressionAST::Boolean(value) => Ok(value),
        LogicExpressionAST::And { left, right } => {
            let left = evaluate_expression(scope, &left)?;
            let right = evaluate_expression(scope, &right)?;

            Ok(left & right)
        }
        LogicExpressionAST::Or { left, right } => {
            let left = evaluate_expression(scope, &left)?;
            let right = evaluate_expression(scope, &right)?;

            Ok(left | right)
        }
        LogicExpressionAST::Not { expr } => {
            let expr = evaluate_expression(scope, &expr)?;

            Ok(!expr)
        }
    }
}

impl ops::Not for ValueType {
    type Output = bool;

    fn not(self) -> Self::Output {
        value_to_bool(self).not()
    }
}

impl ops::BitAnd for ValueType {
    type Output = bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        value_to_bool(self) & value_to_bool(rhs)
    }
}

impl ops::BitOr for ValueType {
    type Output = bool;

    fn bitor(self, rhs: Self) -> Self::Output {
        value_to_bool(self) | value_to_bool(rhs)
    }
}

pub fn value_to_bool(value: ValueType) -> bool {
    match value {
        ValueType::Variable(value) => primitive_types_to_bool(value),
        ValueType::Function(_) => true,
        ValueType::InternalFunction(_) => true,
        ValueType::Module(_) => true,
    }
}

fn primitive_types_to_bool(value: PrimitiveTypes) -> bool {
    match value {
        PrimitiveTypes::Boolean(value) => value,
        PrimitiveTypes::Array(_) => true,
        PrimitiveTypes::Object(_) => true,
        PrimitiveTypes::Null => false,
        PrimitiveTypes::Undefined => false,
        PrimitiveTypes::Number(number) => match number {
            lv8_parser::Either::Left(value) => value != 0,
            lv8_parser::Either::Right(value) => value.is_normal(),
        },
        PrimitiveTypes::String(value) => !value.is_empty(),
    }
}
