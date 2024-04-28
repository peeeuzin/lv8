use std::collections::HashMap;

use lv8_parser::{ASTNode, Either};

mod block;
mod expression;
mod function;
mod scope;
mod statement;
mod stdlib;

#[derive(Clone, Debug)]
pub enum PrimitiveTypes {
    Null,
    Undefined,
    Boolean(bool),
    Number(Either<i64, f64>),
    String(String),
    Array(Vec<PrimitiveTypes>),
    Object(HashMap<String, PrimitiveTypes>),
    Identifier(String),
}

impl ToString for PrimitiveTypes {
    fn to_string(&self) -> String {
        match self {
            PrimitiveTypes::Null => "null".to_string(),
            PrimitiveTypes::Undefined => "undefined".to_string(),
            PrimitiveTypes::Boolean(value) => value.to_string(),
            PrimitiveTypes::Number(value) => value.to_string(),
            PrimitiveTypes::String(value) => value.to_string(),
            PrimitiveTypes::Array(value) => {
                let mut array = Vec::new();

                for element in value {
                    array.push(element.to_string());
                }

                format!("[{}]", array.join(", "))
            }
            PrimitiveTypes::Object(value) => {
                let mut object = Vec::new();

                for (key, value) in value {
                    object.push(format!("{}: {}", key, value.to_string()));
                }

                format!("{{{}}}", object.join(", "))
            }
            PrimitiveTypes::Identifier(value) => value.to_string(),
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
