window.highlighterInfo = {
    queryString: `"break" @keyword
"commutative" @keyword
"continue" @keyword
"else" @keyword
"fn" @keyword
"if" @keyword
"let" @keyword
"operator" @keyword
"pub" @keyword
"return" @keyword
"struct" @keyword
"static" @keyword
"watch" @keyword
"while" @keyword
"-----constraints-----" @keyword
"-----impl-----" @keyword
"-----static-----" @keyword

(number_literal) @number
(string_literal) @string
(bool_literal) @bool


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
    },
};
