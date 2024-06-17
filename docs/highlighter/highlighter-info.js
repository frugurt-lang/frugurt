window.highlighterInfo = {
    queryString: `[
  "break"
  "class"
  "commutative"
  "continue"
  "data"
  "else"
  "fn"
  "if"
  "impl"
  "import"
  "let"
  "operator"
  "pub"
  "return"
  "scope"
  "struct"
  "static"
  "while"
] @keyword

(number_literal) @number
(string_literal) @string
(bool_literal)   @bool
(nah_literal)    @nah
(comment)        @comment

(let_statement
    ident: (identifier) @function.declaration
    value: [
        (function_expression)
        (curry_call_expression)
    ]
)
`,
    colors:      {
        "keyword":              "#CF8E6D",
        "function.declaration": "#56A8F5",
        "number":               "#2AACB8",
        "string":               "#6AAB73",
        "bool":                 "#CF8E6D",
        "nah":                  "#CF8E6D",
        "comment":              "#7A7E85",
    },
};
