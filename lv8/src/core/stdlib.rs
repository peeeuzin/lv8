use std::{collections::HashMap, io::Write};

use super::{scope::ValueType, PrimitiveTypes};

pub fn build_standard_library() -> HashMap<String, ValueType> {
    let mut standard_library = HashMap::new();

    standard_library.insert("printl".to_string(), ValueType::InternalFunction(printl));
    standard_library.insert("print".to_string(), ValueType::InternalFunction(print));
    standard_library.insert("input".to_string(), ValueType::InternalFunction(input));

    standard_library
}

fn printl(args: Vec<ValueType>) -> ValueType {
    let args = args
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        + "\n";

    std::io::stdout().write_all(args.as_bytes()).unwrap();

    ValueType::Variable(PrimitiveTypes::Undefined)
}

fn print(args: Vec<ValueType>) -> ValueType {
    let args = args
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    std::io::stdout().write_all(args.as_bytes()).unwrap();

    ValueType::Variable(PrimitiveTypes::Undefined)
}

fn input(_args: Vec<ValueType>) -> ValueType {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    ValueType::Variable(PrimitiveTypes::String(input.trim().to_string()))
}
