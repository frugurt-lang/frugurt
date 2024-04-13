use anyhow::Error;
use mdbook::{
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
};

#[derive(Default)]
pub struct FruHighlight;

impl FruHighlight {
    pub fn new() -> FruHighlight {
        FruHighlight
    }
}

impl Preprocessor for FruHighlight {
    fn name(&self) -> &str {
        "fru-highlight"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let mdbook::BookItem::Chapter(chapter) = item {
                chapter.content = highlight_code(&chapter.content);
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}


fn highlight_code(code: &str) -> String {
    let mut result = "<style>
    .keyword {
        color: #8b5d0a;
    }
</style>\n".to_string();

    // seeking for "```frugurt" ...code... "```"

    let mut it = code.chars();

    while let Some(c) = it.next() {
        if it.as_str().starts_with("```frugurt") {
            it.advance_by(10).unwrap();

            let mut code = String::default();

            while let Some(c) = it.next() {
                if it.as_str().starts_with("```") {
                    break;
                }
                code.push(c);
            }

            result.push_str("<pre>");
            result.push_str(&highlight_code_block(code));
            result.push_str("</pre>");

            it.advance_by(3).unwrap()
        } else {
            result.push(c);
        }
    }

    result
}

fn highlight_code_block(mut code: String) -> String {
    let keywords = ["nah", "true", "false", "let", "while", "return", "if", "else", "fn", "operator",
        "commutative", "break", "continue", "struct", "pub", "static", "watch", "-----constraints-----",
        "-----impl-----", "-----static-----"];


    for kw in keywords {
        code = code.replace(kw, format!("<span class=\"keyword\">{}</span>", kw).as_str());
    }

    code
}

#[cfg(test)]
mod test {
    use crate::highlight::highlight_code_block;

    #[test]
    fn test_highlight_block() {
        let code = r#"
        let a = 1;
        struct Box {
            x;
        }

        if a == 1 {
            return true;
        }
        "#;

        println!("{}", highlight_code_block(code.to_string()));
    }
}
