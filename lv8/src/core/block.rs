use super::{
    scope::{self, Scope, ValueType},
    statement,
};
use lv8_parser::Block as BlockAST;

#[derive(Clone, Debug)]
pub struct Block {
    block: BlockAST,
    pub scope: Scope,
}

impl Block {
    pub fn new(block: BlockAST, scope: Scope) -> Self {
        Self { block, scope }
    }

    pub fn return_type(&self) -> ValueType {
        let return_statement = &self.block.1;

        match return_statement {
            lv8_parser::ReturnStatement::Return(expression) => {
                scope::expression_to_value(&self.scope, expression)
            }
            lv8_parser::ReturnStatement::Break => {
                ValueType::Variable(super::PrimitiveTypes::Undefined)
            }
            lv8_parser::ReturnStatement::Continue => {
                ValueType::Variable(super::PrimitiveTypes::Undefined)
            }
        }
    }

    pub fn call(mut self) -> ValueType {
        let statements = &self.block.0;

        for statement in statements {
            statement::run_statement(&mut self.scope, statement);
        }

        self.return_type()
    }
}
