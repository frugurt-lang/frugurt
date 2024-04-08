#![feature(try_trait_v2)]
#![feature(iterator_try_collect)]
#![feature(linked_list_cursors)]

use std::{path::PathBuf, time::Instant};

use clap::Parser;

pub use crate::interpreter::runner::execute_file;

mod helpers;
mod interpreter;

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

#[derive(Parser, Debug)]
struct Args {
    #[clap(required = true, help = "File to execute")]
    filename: PathBuf,

    #[clap(name = "converter", short, long, help = "Path to converter.exe")]
    converter_path: Option<PathBuf>,

    #[clap(short, long, help = "Print execution time")]
    time: bool,
}

fn main() {
    let args: Args = Args::parse();

    let start = Instant::now();

    let result = execute_file(
        args.filename.as_path(),
        match &args.converter_path {
            Some(path) => Some(path.as_path()),
            None => None,
        },
    );

    if let Err(err) = result {
        println!("{}", err);
    }

    if args.time {
        println!("Program finished in {}ms", start.elapsed().as_millis());
    }
}
