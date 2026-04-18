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

; Access and member operations
(member_access) @property
(index_access) @function
(map_entry) @property

; Struct operations
(struct_literal) @function
(struct_field) @property
(struct_method) @function

; Function calls
(function_call) @function

; Function definitions
(function_literal) @function

; Builtin functions
(builtin) @function.builtin
"@" @function.builtin

; Blocks and parameters
(block) @punctuation.bracket
(parameter_list) @punctuation.bracket

; Keywords
"if" @keyword
"else" @keyword
"return" @keyword
"while" @keyword
"struct" @keyword
"true" @constant.builtin
"false" @constant.builtin

(if_statement) @keyword
(else_if_statement) @keyword
(else_statement) @keyword
(while_statement) @keyword
(return_statement) @keyword

; Operators - binary/comparison
(greater) @operator
(less) @operator
(greater_equal) @operator
(less_equal) @operator
(equal) @operator
(not_equal) @operator
(and) @operator
(or) @operator

; Operators - arithmetic
(plus) @operator
(minus) @operator
(times) @operator
(divide) @operator
(modulus) @operator

; Operators - unary/access
(unary_expression) @operator
(dereference) @operator

; Assignment operators
(assignment_operator) @operator
"=" @operator

; Range operators
(range_all) @operator
(range_to) @operator
(range_to_include) @operator

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
