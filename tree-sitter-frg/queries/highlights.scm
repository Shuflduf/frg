; Most of these are copied from rust's highlights

; Comments
(comment) @comment

; Types - primitives
"void" @type.builtin
"int" @type.builtin
"float" @type.builtin
"str" @type.builtin
"bool" @type.builtin

; Types - collections
"vec" @type.builtin
"arr" @type.builtin
"array" @type.builtin
"list" @type.builtin
"map" @type.builtin
"obj" @type.builtin
"hashmap" @type.builtin
"dict" @type.builtin
"dictionary" @type.builtin
"struct" @keyword

; Type nodes
(type) @type
(vec_type) @type
(map_type) @type
(function_type) @type
(reference_type) @type
(struct_identifier) @type

; Literals
(int_literal) @constant.builtin
(float_literal) @constant.builtin
(string_literal) @string
(bool_literal) @constant.builtin

; Identifiers
(identifier) @variable
; (num_identifier) @variable.other.member
(struct_field (identifier) @variable.other.member)

; Identifier conventions - uppercase names are types/constructors
((identifier) @constructor
 (#match? @constructor "^[A-Z]"))

; Collections
(vec_literal) @attribute
(map_literal) @attribute
(empty_collection) @attribute

; Function calls
(function_call) @function

; Function definitions
(function_literal) @function

; Builtin functions
(builtin) @function.builtin
"@" @function.builtin

; Keywords
"if" @keyword
"else" @keyword
"return" @keyword

(if_statement) @keyword
(else_if_statement) @keyword
(else_statement) @keyword

; Operators
"=" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
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
"!" @operator
"&" @operator

; Range operators
".." @operator

; Delimiters
"," @punctuation.delimiter
"." @punctuation.delimiter
":" @punctuation.delimiter

; Brackets
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
