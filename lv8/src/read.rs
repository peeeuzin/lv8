use std::fs;

use lv8_parser::ASTNode;

pub fn read_file(path: &str) -> ASTNode {
    let input = fs::read_to_string(path).unwrap();

    lv8_parser::parse(&input).unwrap()
}
