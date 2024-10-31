use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

mod comparison_expression;
mod logic_expression;
mod math_expression;

use lv8_parser::Expression as ExpressionAST;

pub use logic_expression::value_to_bool;

use super::{scope::Scope, PrimitiveTypes};

pub struct Expression;

impl Expression {
    pub fn parse_expression(scope: &Rc<RefCell<Scope>>, ast: ExpressionAST) -> PrimitiveTypes {
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
                let mut object = BTreeMap::new();

                for (key, value) in value {
                    object.insert(key, Expression::parse_expression(scope, value));
                }

                PrimitiveTypes::Object(object)
            }
            ExpressionAST::Identifier(value) => PrimitiveTypes::String(value),
            ExpressionAST::MathExpression(value) => {
                math_expression::evaluate_math_expression(scope, value)
            }
            ExpressionAST::LogicExpression(value) => {
                PrimitiveTypes::Boolean(logic_expression::evaluate_logic_expression(scope, value))
            }
            ExpressionAST::ComparisonExpression(value) => PrimitiveTypes::Boolean(
                comparison_expression::evaluate_comparison_expression(scope, value),
            ),
        }
    }
}
