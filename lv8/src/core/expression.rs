use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

mod comparison_expression;
mod logic_expression;
mod math_expression;
mod namespace_expression;

use lv8_common::error::Result;
use lv8_parser::Expression as ExpressionAST;

pub use logic_expression::value_to_bool;

use super::{
    scope::{evaluate_expression, Scope, ValueType},
    PrimitiveTypes,
};

pub struct Expression;

impl Expression {
    pub fn parse_expression(scope: &Rc<RefCell<Scope>>, ast: ExpressionAST) -> Result<ValueType> {
        let result = match ast {
            ExpressionAST::Null => ValueType::Variable(PrimitiveTypes::Null),
            ExpressionAST::Undefined => ValueType::Variable(PrimitiveTypes::Undefined),
            ExpressionAST::Boolean(value) => ValueType::Variable(PrimitiveTypes::Boolean(value)),
            ExpressionAST::Number(value) => ValueType::Variable(PrimitiveTypes::Number(value)),
            ExpressionAST::String(value) => ValueType::Variable(PrimitiveTypes::String(value)),
            ExpressionAST::Array(value) => {
                let mut array = Vec::new();

                for element in value {
                    array.push(evaluate_expression(scope, &element)?);
                }

                ValueType::Variable(PrimitiveTypes::Array(array))
            }
            ExpressionAST::Object(value) => {
                let mut object = BTreeMap::new();

                for (key, value) in value {
                    object.insert(key, evaluate_expression(scope, &value)?);
                }

                ValueType::Variable(PrimitiveTypes::Object(object))
            }
            // ExpressionAST::Identifier(value) => evaluate_expression(scope, &ast),
            ExpressionAST::Identifier(value) => ValueType::Variable(PrimitiveTypes::String(value)),
            ExpressionAST::Namespace(value) => {
                namespace_expression::evaluate_namespace_expression(scope, value)?
            }
            ExpressionAST::MathExpression(value) => {
                ValueType::Variable(math_expression::evaluate_math_expression(scope, value)?)
            }
            ExpressionAST::LogicExpression(value) => ValueType::Variable(PrimitiveTypes::Boolean(
                logic_expression::evaluate_logic_expression(scope, value)?,
            )),
            ExpressionAST::ComparisonExpression(value) => {
                ValueType::Variable(PrimitiveTypes::Boolean(
                    comparison_expression::evaluate_comparison_expression(scope, value)?,
                ))
            }
        };

        Ok(result)
    }
}
