use super::{
    scope::{self, Scope, VariableType},
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

    pub fn return_type(&self) -> VariableType {
        let return_statement = &self.block.1;

        match return_statement {
            lv8_parser::ReturnStatement::Return(expression) => {
                scope::expression_to_value(&self.scope, expression)
            }
            lv8_parser::ReturnStatement::Break => {
                VariableType::Variable(super::PrimitiveTypes::Undefined)
            }
            lv8_parser::ReturnStatement::Continue => {
                VariableType::Variable(super::PrimitiveTypes::Undefined)
            }
        }
    }

    pub fn call(mut self) -> VariableType {
        let statements = &self.block.0;

        for statement in statements {
            statement::run_statement(&mut self.scope, statement);
        }

        self.return_type()
    }
}
