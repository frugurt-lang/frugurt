use clap::Parser;

use interpreter::interpreter::Interpreter;
use std::{
    fs::read_to_string,
    path::PathBuf,
    time::Instant,
};

#[derive(Parser, Debug)]
struct Args {
    #[clap(required = true, help = "File to execute")]
    filename: PathBuf,

    #[clap(short, long, help = "Print execution time")]
    time: bool,
}

fn main() {
    let args: Args = Args::parse();

    let start = Instant::now();

    let path = args.filename.as_path();

    let source_code = match read_to_string(path) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error reading file {path:?} \n {err}");
            std::process::exit(1);
        }
    };

    let interpreter = Interpreter::new();

    let result = interpreter.execute_code(source_code);

    if let Err(err) = &result {
        eprintln!("{}", err);
    }

    if args.time {
        println!("Program finished in {}ms", start.elapsed().as_millis());
    }

    if result.is_err() {
        std::process::exit(1);
    }
}
