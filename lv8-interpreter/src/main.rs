mod core;
mod error;
mod read;

use clap::Parser;
use core::Core;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,

    /// Show the execution time of the program
    #[clap(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let ast = read::read_file(&args.path);

    let core = Core::new(ast);

    if args.debug {
        execute_with_debug(core)
    } else {
        core.execute();
    }
}

fn measure_execution_time<F: FnOnce() -> R, R>(f: F) -> (R, std::time::Duration) {
    let start = std::time::Instant::now();
    let result = f();
    let end = std::time::Instant::now();

    (result, end - start)
}

fn execute_with_debug(core: Core) {
    let (_, duration) = measure_execution_time(|| core.execute());

    println!("Took: {:?}", duration);
}
