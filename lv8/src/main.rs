mod core;
mod error;
mod read;
mod repl;

use clap::Parser;
use core::Core;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.path {
        execute_file(&path);
    } else {
        repl::run().unwrap();
    }
}

fn execute_file(path: &str) {
    let ast = read::read_file(path);
    let core = Core::new();
    core.execute(ast);
}
