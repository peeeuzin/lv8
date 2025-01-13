use std::{fs, path::Path};

use lv8_common::error::Result;
use lv8_parser::ASTNode;

pub fn read_file<P>(path: &P) -> Result<ASTNode>
where
    P: AsRef<Path>,
{
    let input = fs::read_to_string(path).unwrap();

    lv8_parser::parse(&input)
}
