use std::ops;

use lv8_parser::LogicExpression as LogicExpressionAST;

use crate::core::{
    scope::{expression_to_value, Scope, ValueType},
    PrimitiveTypes,
};

pub fn evaluate_logic_expression(scope: &Scope, logic_expression: LogicExpressionAST) -> bool {
    match logic_expression {
        LogicExpressionAST::Boolean(value) => value,
        LogicExpressionAST::And { left, right } => {
            let left = expression_to_value(scope, &left);
            let right = expression_to_value(scope, &right);

            left & right
        }
        LogicExpressionAST::Or { left, right } => {
            let left = expression_to_value(scope, &left);
            let right = expression_to_value(scope, &right);

            left | right
        }
        LogicExpressionAST::Not { expr } => !expression_to_value(scope, &expr),
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
