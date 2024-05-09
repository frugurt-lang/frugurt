#![feature(iterator_try_collect)]

mod helpers;
mod interpreter;

use std::{path::PathBuf, time::Instant};

use clap::Parser;

use crate::interpreter::runner::execute_file;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

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

    let result = execute_file(args.filename.as_path());

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
