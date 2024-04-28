use std::collections::HashMap;

mod math_expression;

use lv8_parser::Expression as ExpressionAST;

use super::{scope::Scope, PrimitiveTypes};

pub struct Expression;

impl Expression {
    pub fn parse_expression(scope: &Scope, ast: ExpressionAST) -> PrimitiveTypes {
        match ast {
            ExpressionAST::Null => PrimitiveTypes::Null,
            ExpressionAST::Undefined => PrimitiveTypes::Undefined,
            ExpressionAST::Boolean(value) => PrimitiveTypes::Boolean(value),
            ExpressionAST::Number(value) => PrimitiveTypes::Number(value),
            ExpressionAST::String(value) => PrimitiveTypes::String(value),
            ExpressionAST::Array(value) => {
                let mut array = Vec::new();

                for element in value {
                    array.push(Expression::parse_expression(scope, element));
                }

                PrimitiveTypes::Array(array)
            }
            ExpressionAST::Object(value) => {
                let mut object = HashMap::new();

                for (key, value) in value {
                    object.insert(key, Expression::parse_expression(scope, value));
                }

                PrimitiveTypes::Object(object)
            }
            ExpressionAST::Identifier(value) => PrimitiveTypes::Identifier(value),
            ExpressionAST::MathExpression(value) => {
                math_expression::evaluate_math_expression(scope, value)
            }
        }
    }
}
