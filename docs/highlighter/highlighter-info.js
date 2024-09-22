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

(nah_literal)    @nah
(bool_literal)   @bool
(number_literal) @number
(string_literal) @string
(comment)        @comment

(let_statement
    ident: (identifier) @function.declaration
    value: (function_expression)
)
`,
    colors:      {
        "keyword":              "#CF8E6D",
        "function.declaration": "#56A8F5",
        "nah":                  "#CF8E6D",
        "bool":                 "#CF8E6D",
        "number":               "#2AACB8",
        "string":               "#6AAB73",
        "comment":              "#7A7E85",
    },
};
