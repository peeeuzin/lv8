use crate::core::{
    scope::{self, Scope},
    PrimitiveTypes,
};
use lv8_parser::{Either, MathExpression, MathOperation};

pub fn evaluate_math_expression(scope: &Scope, math_expression: MathExpression) -> PrimitiveTypes {
    match math_expression {
        MathExpression::Number(value) => PrimitiveTypes::Number(value),
        MathExpression::Operation {
            left,
            operation,
            right,
        } => {
            let left = match scope::expression_to_value(scope, &left) {
                scope::ValueType::Variable(PrimitiveTypes::Number(value)) => value,
                _ => unreachable!("unreachable!() in math_expression.rs, left"),
            };

            let right = match scope::expression_to_value(scope, &right) {
                scope::ValueType::Variable(PrimitiveTypes::Number(value)) => value,
                _ => unreachable!("unreachable!() in math_expression.rs, left"),
            };

            match operation {
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
                    _ => unreachable!("unreachable!() in math_expression.rs, Exponentiation"),
                },
            }
        }
    }
}
