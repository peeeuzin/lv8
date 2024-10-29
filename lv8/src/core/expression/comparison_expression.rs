use lv8_parser::{ComparisonExpression as ComparisonExpressionAST, ComparisonOperation};

use crate::core::scope::{expression_to_value, Scope};

pub fn evaluate_comparison_expression(
    scope: &Scope,
    comparision_expression: ComparisonExpressionAST,
) -> bool {
    let left = expression_to_value(scope, &comparision_expression.left);
    let right = expression_to_value(scope, &comparision_expression.right);

    match comparision_expression.operation {
        ComparisonOperation::Equal => left == right,
        ComparisonOperation::NotEqual => left != right,
        ComparisonOperation::GreaterThan => left > right,
        ComparisonOperation::LessThan => left < right,
        ComparisonOperation::GreaterThanOrEqual => left >= right,
        ComparisonOperation::LessThanOrEqual => left <= right,
    }
}
