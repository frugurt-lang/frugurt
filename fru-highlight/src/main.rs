use std::{
    fs::OpenOptions,
    io::{self, Write},
    process::exit,
};

use clap::{Arg, ArgMatches, Command};
use mdbook::{
    book::Book,
    BookItem,
    preprocess::{
        CmdPreprocessor,
        Preprocessor,
        PreprocessorContext,
    },
    errors::Error,
};
use semver::{Version, VersionReq};

pub fn make_app() -> Command {
    Command::new("nop-preprocessor")
        .about("A mdbook preprocessor which does precisely nothing")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    // Users will want to construct their own preprocessor here
    let preprocessor = FruHighlighter::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    if supported {
        exit(0);
    } else {
        exit(1);
    }
}


struct FruHighlighter;

impl FruHighlighter {
    fn new() -> Self {
        FruHighlighter
    }
}

impl Preprocessor for FruHighlighter {
    fn name(&self) -> &str {
        "fru-highlight"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> mdbook::errors::Result<Book> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("chapter-lists.txt")?;

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                file.write_all(chapter.name.as_ref()).expect("TODO: panic message");
            }
        });


        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
