(comment) @comment

"void" @type
"int" @type
"float" @type
"str" @type
"bool" @type
"vec" @type
"arr" @type
"array" @type
"list" @type
"map" @type
"obj" @type
"hashmap" @type
"dict" @type
"dictionary" @type
"struct" @type
; "set" @type

(type) @type
(vec_type) @type
(map_type) @type
(function_type) @type
(reference_type) @type
(struct_identifier) @type

(int_literal) @number
(float_literal) @number
(string_literal) @string
(bool_literal) @boolean

(identifier) @variable
(num_identifier) @variable

(vec_literal) @attribute
(map_literal) @attribute
(empty_collection) @attribute

(function_literal) @attribute
(function_call) @attribute
(builtin) @attribute

"struct" @keyword
"return" @keyword
"if" @keyword
"else" @keyword

(if_statement) @conditional
(else_if_statement) @conditional
(else_statement) @conditional

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

; "if" @conditional
; "else" @conditional
; "return" @keyword

(builtin) @function.builtin
; "@" @function.builtin
