(comment) @comment

(type) @type
"void" @type
"int" @type
"float" @type
"str" @type
"bool" @type
"range" @type
"vec" @type
"set" @type
"map" @type
"struct" @type

(int_literal) @number
(float_literal) @number
(string_literal) @string
(bool_literal) @boolean

(identifier) @variable

"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
">" @operator
"<" @operator
">=" @operator
"<=" @operator
"==" @operator
"!=" @operator
"&&" @operator
"||" @operator
; "..=" @operator
; ".. " @operator
; ".." @operator
"!" @operator

"," @punctuation.delimiter
"." @punctuation.delimiter
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

; (function_type (@function)
(parameter_declaration (identifier) @parameter)
; (function_call (identifier) @function.call)
(function_literal) @function

"if" @conditional
"else" @conditional
"return" @keyword

(builtin) @function.builtin
"@" @function.builtin
