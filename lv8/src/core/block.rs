use std::{cell::RefCell, rc::Rc};

use super::{
    scope::{self, Scope, ValueType},
    statement, Metadata,
};
use lv8_common::error::Result;
use lv8_parser::Block as BlockAST;

#[derive(Clone, Debug)]
pub struct Block {
    block: BlockAST,
    pub scope: Rc<RefCell<Scope>>,
    metadata: Rc<Metadata>,
}

impl Block {
    pub fn new(block: BlockAST, scope: Rc<RefCell<Scope>>, metadata: Rc<Metadata>) -> Self {
        Self {
            block,
            scope,
            metadata,
        }
    }

    pub fn return_type(&self) -> Result<ValueType> {
        let return_statement = &self.block.1;

        match return_statement {
            lv8_parser::ReturnStatement::Return(expression) => {
                scope::evaluate_expression(&self.scope, expression)
            }
            lv8_parser::ReturnStatement::Break => {
                Ok(ValueType::Variable(super::PrimitiveTypes::Undefined))
            }
            lv8_parser::ReturnStatement::Continue => {
                Ok(ValueType::Variable(super::PrimitiveTypes::Undefined))
            }
        }
    }

    pub fn call(self) -> Result<ValueType> {
        let statements = &self.block.0;

        for statement in statements {
            statement::run_statement(&self.scope, statement, &self.metadata)?;
        }

        self.return_type()
    }
}
