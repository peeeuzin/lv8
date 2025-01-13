mod core;
mod read;
mod repl;

use clap::Parser;
use core::execute_file;
use lv8_common::error::Result;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(path) = args.path {
        execute_file(path)?;
    } else {
        repl::run().unwrap();
    }

    Ok(())
}
