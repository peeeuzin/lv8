use std::{cell::RefCell, rc::Rc};

use lv8_common::error::Result;
use lv8_parser::{ComparisonExpression as ComparisonExpressionAST, ComparisonOperation};

use crate::core::scope::{evaluate_expression, Scope};

pub fn evaluate_comparison_expression(
    scope: &Rc<RefCell<Scope>>,
    comparision_expression: ComparisonExpressionAST,
) -> Result<bool> {
    let left = evaluate_expression(scope, &comparision_expression.left)?;
    let right = evaluate_expression(scope, &comparision_expression.right)?;

    let result = match comparision_expression.operation {
        ComparisonOperation::Equal => left == right,
        ComparisonOperation::NotEqual => left != right,
        ComparisonOperation::GreaterThan => left > right,
        ComparisonOperation::LessThan => left < right,
        ComparisonOperation::GreaterThanOrEqual => left >= right,
        ComparisonOperation::LessThanOrEqual => left <= right,
    };

    Ok(result)
}
