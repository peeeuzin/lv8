use std::collections::HashMap;

use super::{scope::ValueType, PrimitiveTypes};

pub fn build_standard_library() -> HashMap<String, ValueType> {
    let mut standard_library = HashMap::new();

    standard_library.insert("print".to_string(), ValueType::InternalFunction(print));

    standard_library
}

fn print(args: Vec<ValueType>) -> ValueType {
    let args = args
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", args);

    ValueType::Variable(PrimitiveTypes::Undefined)
}
