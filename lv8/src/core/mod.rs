use lv8_common::error::Result;
use lv8_parser::{ASTNode, Either};
use owo_colors::OwoColorize;
use scope::{Scope, ValueType};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::{self, Debug};
use std::path::{self, Path};
use std::rc::Rc;

use crate::read;

mod block;
mod expression;
mod flow_control;
mod function;
mod import;
mod module;
mod scope;
mod statement;
mod stdlib;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum PrimitiveTypes {
    Null,
    Undefined,
    Boolean(bool),
    Number(Either<isize, f64>),
    String(String),
    Array(Vec<ValueType>),
    Object(BTreeMap<String, ValueType>),
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
                write!(f, "{:#?}", value)
            }
            PrimitiveTypes::Object(value) => {
                write!(f, "{:#?}", value)
            }
        }
    }
}

impl Debug for PrimitiveTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveTypes::Number(value) => match value {
                Either::Left(value) => write!(f, "{}", value.yellow()),
                Either::Right(value) => write!(f, "{:?}f", value.yellow()),
            },
            PrimitiveTypes::String(value) => write!(f, "{}", format!("\"{}\"", value).green()),
            PrimitiveTypes::Undefined => write!(f, "{}", "undefined".bright_black()),
            PrimitiveTypes::Null => write!(f, "{}", "null".bright_black()),
            PrimitiveTypes::Boolean(value) => write!(f, "{}", value.bright_blue()),
            _ => write!(f, "{}", self),
        }
    }
}

#[derive(Debug)]
pub struct Metadata {
    pub pw: String,
}

pub struct Evaluator {
    pub scope: Rc<RefCell<Scope>>,
    pub metadata: Rc<Metadata>,
}

impl Evaluator {
    pub fn new(metadata: Metadata) -> Self {
        let mut scope = scope::Scope::new("global");

        let standard_library = stdlib::build_standard_library();
        scope.extend(standard_library);

        Self {
            scope: Rc::new(RefCell::new(scope)),
            metadata: Rc::new(metadata),
        }
    }

    pub fn execute(&self, ast: ASTNode) -> Result<ValueType> {
        match ast {
            ASTNode::Block(block) => {
                let block = block::Block::new(block, self.scope.clone(), self.metadata.clone());

                block.call()
            }
        }
    }
}

pub fn execute_file<P>(path: P) -> Result<Evaluator>
where
    P: AsRef<Path>,
{
    let ast = read::read_file(&path)?;
    let core = Evaluator::new(Metadata {
        pw: path::absolute(path)
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    });

    match core.execute(ast) {
        Ok(_) => {}
        Err(e) => eprintln!("{:?}", e),
    }

    Ok(core)
}
