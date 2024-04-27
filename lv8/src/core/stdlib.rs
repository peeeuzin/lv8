use std::collections::HashMap;

use super::{scope::VariableType, PrimitiveTypes};

pub fn build_standard_library() -> HashMap<String, VariableType> {
    let mut standard_library = HashMap::new();

    standard_library.insert("print".to_string(), VariableType::InternalFunction(print));
    standard_library.insert(
        "mark.zuck".to_string(),
        VariableType::Variable(PrimitiveTypes::String("Mark Zuckerberg".to_string())),
    );

    standard_library
}

fn print(args: Vec<VariableType>) -> VariableType {
    let args = args
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", args);

    VariableType::Variable(PrimitiveTypes::Undefined)
}
