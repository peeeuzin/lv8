use crate::core::{Evaluator, Metadata};
use rustyline::{DefaultEditor, Result};

pub fn run() -> Result<()> {
    println!("Welcome to LV8 {}", env!("CARGO_PKG_VERSION"));
    println!("Type 'exit' or Ctrl+C. to exit");
    println!();

    let mut rl = DefaultEditor::new().unwrap();

    let core = Evaluator::new(Metadata {
        pw: std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    });

    let mut i = 0;
    loop {
        let readline = rl.readline(&format!("LV8({})> ", i));

        match readline {
            Ok(line) => {
                if line.trim() == "exit" {
                    break;
                }

                i += 1;
                rl.add_history_entry(line.as_str()).ok();
                let ast = match lv8_parser::parse(&line) {
                    Ok(ast) => ast,
                    Err(err) => {
                        eprintln!("{:?}", err);
                        continue;
                    }
                };

                match core.execute(ast) {
                    Ok(result) => {
                        eprintln!("{:?}", result);
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }

            _ => break,
        }
    }

    Ok(())
}
