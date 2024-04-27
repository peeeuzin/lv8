use pest::iterators::Pair;

use super::{error::Result, Expression, Rule};

pub fn parse(pair: Pair<Rule>) -> Result<Expression> {
    match pair.as_rule() {
        Rule::null => Ok(Expression::Null),
        Rule::undefined => Ok(Expression::Undefined),
        Rule::boolean => Ok(Expression::Boolean(pair.as_str().parse().unwrap())),
        Rule::float => Ok(Expression::Float(pair.as_str().parse().unwrap())),
        Rule::integer => Ok(Expression::Integer(pair.as_str().parse().unwrap())),
        Rule::string => {
            let inner = pair.into_inner().next().unwrap();
            Ok(Expression::String(inner.as_str().to_string()))
        }
        Rule::object => {
            let mut object = std::collections::HashMap::new();

            for pair in pair.into_inner() {
                let mut inner = pair.into_inner();

                let key = match parse(inner.next().unwrap())? {
                    Expression::String(key) => key,
                    Expression::Identifier(key) => key,
                    _ => unreachable!(),
                };

                let value = parse(inner.next().unwrap())?;
                object.insert(key, value);
            }

            Ok(Expression::Object(object))
        }
        Rule::array => {
            let mut array = Vec::new();

            for pair in pair.into_inner() {
                array.push(parse(pair)?);
            }

            Ok(Expression::Array(array))
        }
        Rule::ident => Ok(Expression::Identifier(pair.as_str().to_string())),
        Rule::expr => {
            let inner = pair.into_inner().next().unwrap();
            Ok(parse(inner)?)
        }
        _ => unreachable!("unreachable!() in expression.rs, {:?}", pair.as_rule()),
    }
}
