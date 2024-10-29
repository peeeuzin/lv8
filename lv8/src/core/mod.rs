use lv8_parser::{ASTNode, Either};
use std::collections::BTreeMap;
use std::fmt;

mod block;
mod expression;
mod function;
mod scope;
mod statement;
mod stdlib;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum PrimitiveTypes {
    Null,
    Undefined,
    Boolean(bool),
    Number(Either<i64, f64>),
    String(String),
    Array(Vec<PrimitiveTypes>),
    Object(BTreeMap<String, PrimitiveTypes>),
}

impl fmt::Display for PrimitiveTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveTypes::Null => write!(f, "null"),
            PrimitiveTypes::Undefined => write!(f, "undefined"),
            PrimitiveTypes::Boolean(value) => write!(f, "{}", value),
            PrimitiveTypes::Number(value) => write!(f, "{:?}", value),
            PrimitiveTypes::String(value) => write!(f, "{}", value),
            PrimitiveTypes::Array(value) => {
                let mut array = Vec::new();

                for element in value {
                    array.push(element.to_string());
                }

                write!(f, "[{}]", array.join(", "))
            }
            PrimitiveTypes::Object(value) => {
                let mut object = Vec::new();

                for (key, value) in value {
                    object.push(format!("\"{}\": {}", key, value));
                }

                write!(f, "{{{}}}", object.join(", "))
            }
        }
    }
}

pub struct Core {
    pub ast: ASTNode,
}

impl Core {
    pub fn new(ast: ASTNode) -> Self {
        Self { ast }
    }

    pub fn execute(self) {
        match self.ast {
            ASTNode::Block(block) => {
                let mut scope = scope::Scope::new();

                let standard_library = stdlib::build_standard_library();
                scope.extend_variables(standard_library);

                let block = block::Block::new(block, scope);

                block.call();
            }
        }
    }
}
