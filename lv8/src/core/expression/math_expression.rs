use std::{cell::RefCell, rc::Rc};

use crate::core::{
    scope::{self, Scope},
    PrimitiveTypes,
};
use lv8_common::error::Result;
use lv8_parser::{Either, MathExpression, MathOperation};

pub fn evaluate_math_expression(
    scope: &Rc<RefCell<Scope>>,
    math_expression: MathExpression,
) -> Result<PrimitiveTypes> {
    match math_expression {
        MathExpression::Number(value) => Ok(PrimitiveTypes::Number(value)),
        MathExpression::Operation {
            left,
            operation,
            right,
        } => {
            let left = match scope::evaluate_expression(scope, &left)? {
                scope::ValueType::Variable(PrimitiveTypes::Number(value)) => value,
                _ => unreachable!("unreachable!() in math_expression.rs, left"),
            };

            let right = match scope::evaluate_expression(scope, &right)? {
                scope::ValueType::Variable(PrimitiveTypes::Number(value)) => value,
                _ => unreachable!("unreachable!() in math_expression.rs, right"),
            };

            let result = match operation {
                MathOperation::Add => PrimitiveTypes::Number(left + right),
                MathOperation::Subtract => PrimitiveTypes::Number(left - right),
                MathOperation::Multiply => PrimitiveTypes::Number(left * right),
                MathOperation::Divide => PrimitiveTypes::Number(left / right),
                MathOperation::Modulus => PrimitiveTypes::Number(left % right),
                MathOperation::Exponentiation => match (left, right) {
                    (Either::Left(left), Either::Left(right)) => {
                        PrimitiveTypes::Number(Either::Left(left.pow(right as u32)))
                    }
                    (Either::Right(left), Either::Right(right)) => {
                        PrimitiveTypes::Number(Either::Right(left.powf(right)))
                    }
                    (Either::Left(left), Either::Right(right)) => {
                        PrimitiveTypes::Number(Either::Left(left.pow(right as u32)))
                    }
                    (Either::Right(left), Either::Left(right)) => {
                        PrimitiveTypes::Number(Either::Right(left.powf(right as f64)))
                    }
                },
            };

            Ok(result)
        }
    }
}
