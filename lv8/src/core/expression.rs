use std::collections::HashMap;

use lv8_parser::Expression as ExpressionAST;

use super::PrimitiveTypes;

pub struct Expression;

impl Expression {
    pub fn parse_expression(ast: ExpressionAST) -> PrimitiveTypes {
        match ast {
            ExpressionAST::Null => PrimitiveTypes::Null,
            ExpressionAST::Undefined => PrimitiveTypes::Undefined,
            ExpressionAST::Boolean(value) => PrimitiveTypes::Boolean(value),
            ExpressionAST::Float(value) => PrimitiveTypes::Float(value),
            ExpressionAST::Integer(value) => PrimitiveTypes::Integer(value),
            ExpressionAST::String(value) => PrimitiveTypes::String(value),
            ExpressionAST::Array(value) => {
                let mut array = Vec::new();

                for element in value {
                    array.push(Expression::parse_expression(element));
                }

                PrimitiveTypes::Array(array)
            }
            ExpressionAST::Object(value) => {
                let mut object = HashMap::new();

                for (key, value) in value {
                    object.insert(key, Expression::parse_expression(value));
                }

                PrimitiveTypes::Object(object)
            }
            ExpressionAST::Identifier(value) => PrimitiveTypes::Identifier(value),
        }
    }
}
