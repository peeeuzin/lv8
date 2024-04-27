use clap::Parser;

mod core;
mod error;
mod read;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();

    let ast = read::read_file(&args.path);

    let core = core::Core::new(ast);
    core.execute();
}
