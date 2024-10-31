use crate::core::Core;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub fn run() -> Result<()> {
    println!("Welcome to LV8!");

    let mut rl = DefaultEditor::new()?;

    let core = Core::new();
    let mut i = 0;
    loop {
        let readline = rl.readline(&format!("LV8({})> ", i));

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).ok();
                let ast = lv8_parser::parse(&line).unwrap();
                let result = core.execute(ast);
                println!("{:?}", result);
                i += 1;
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
