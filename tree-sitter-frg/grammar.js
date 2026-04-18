/**
 * @file i like frogs fr
 * @author Shuflduf <shuflduf@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "frg",

  // extras: ($) => [/\s/, $.comment],

  conflicts: ($) => [
    [$.expression, $.parameter_declaration],
    [$.map_literal, $.empty_collection],
    [$.type, $.struct_method],
    [$.vec_literal],
    [$.type, $.void_statement],
  ],

  rules: {
    source_file: ($) => repeat($.statement),

    statement: ($) =>
      seq(
        choice(
          prec(10, $.variable_declaration),
          $.comment,
          $.if_statement,
          $.return_statement,
          $.struct_declaration,
          $.variable_assignment,
          $.void_statement,
          $.while_statement,
          // prec.dynamic(2, $.expression),
        ),
        repeat(";"),
      ),

    comment: ($) => token(seq("//", /.*/)),

    variable_declaration: ($) =>
      prec.dynamic(100, seq($.type, $.identifier, "=", $.expression)),

    type: ($) =>
      choice(
        "void",
        "int",
        "float",
        "str",
        "bool",
        $.vec_type,
        // $.set_type,
        $.map_type,
        $.function_type,
        $.struct_identifier,
        $.reference_type,
      ),

    vec_type: ($) =>
      seq(choice("vec", "array", "arr", "list"), "(", $.type, ")"),
    map_type: ($) =>
      seq(
        choice("map", "obj", "hashmap", "dict", "dictionary"),
        "(",
        $.type,
        repeat(","),
        $.type,
        ")",
      ),
    // set_type: ($) => seq("set", "(", $.type, ")"),

    function_type: ($) => prec.right(seq($.type, repeat1($.parameter_list))),
    parameter_list: ($) => seq("(", repeat(choice($.type, ",")), ")"),

    identifier: ($) => /[a-z_][a-zA-Z0-9_]*/,
    struct_identifier: ($) => /[A-Z][a-zA-Z0-9_]*/,
    num_identifier: ($) => /[a-zA-Z0-9_]*/,

    reference_type: ($) => prec(1, seq("&", $.type)),

    expression: ($) =>
      choice(
        $.identifier,
        $.binary_expression,
        $.int_literal,
        $.float_literal,
        $.string_literal,
        $.bool_literal,
        $.vec_literal,
        $.map_literal,
        // $.set_literal,
        $.empty_collection,
        $.function_literal,
        $.function_call,
        $.struct_literal,
        $.dereference,
        $.member_access,
        $.index_access,
        $.unary_expression,
        $.range,
        $.builtin,
        $.parenthesized,
      ),

    binary_expression: ($) =>
      choice(
        prec.left(
          6,
          seq(
            $.expression,
            choice(
              $.greater,
              $.less,
              $.greater_equal,
              $.less_equal,
              $.equal,
              $.not_equal,
              $.and,
              $.or,
            ),
            $.expression,
          ),
        ),
        prec.left(11, seq($.expression, choice($.plus, $.minus), $.expression)),
        prec.left(
          12,
          seq($.expression, choice($.times, $.divide, $.modulus), $.expression),
        ),
      ),

    greater: () => />+/,
    less: () => /<+/,
    greater_equal: () => />+=+/,
    less_equal: () => /<+=+/,
    equal: () => /==+/,
    not_equal: () => /!+=+/,
    and: () => /&&+/,
    or: () => /\|\|+/,

    plus: () => /\++/,
    minus: () => "-",
    times: () => /\*+/,
    divide: () => /\/+/,
    modulus: () => /%+/,

    int_literal: (_) => /\d+/,
    float_literal: (_) => /\d+\.\d+/,
    string_literal: (_) => seq('"', /[^"]*/, '"'),
    bool_literal: (_) => choice("true", "false"),

    vec_literal: ($) => seq("[", repeat(choice($.expression, ",")), "]"),
    // set_literal: ($) => seq("{", repeat(choice($.expression, ",")), "}"),
    map_literal: ($) => seq("{", repeat(choice($.map_entry, ",")), "}"),
    map_entry: ($) => seq($.expression, repeat1(":"), $.expression),
    empty_collection: ($) => prec(-1, seq("{", repeat(","), "}")),

    function_literal: ($) => seq($.parameter_declaration, $.block),
    parameter_declaration: ($) =>
      seq("(", repeat(choice($.identifier, ",")), ")"),

    block: ($) =>
      prec(
        5,
        seq(
          "{",
          repeat($.statement),
          optional($.expression),
          // prec.dynamic(100, optional($.expression)),
          "}",
        ),
      ),

    function_call: ($) =>
      prec(15, seq($.expression, "(", repeat(choice($.expression, ",")), ")")),

    unary_expression: ($) =>
      choice(
        prec(14, seq("&", $.expression)),
        prec(13, seq("-", $.expression)),
        prec(13, seq("!", $.expression)),
      ),

    dereference: ($) => prec.left(16, seq($.expression, ".*")),
    member_access: ($) =>
      prec.left(16, seq($.expression, ".", $.num_identifier)),

    index_access: ($) =>
      prec.left(
        12,
        seq($.expression, repeat1("["), $.expression, repeat1("]")),
      ),

    range: ($) =>
      prec.left(
        3,
        seq(
          optional($.expression),
          choice($.range_all, $.range_to, $.range_to_include),
        ),
      ),

    range_all: () => /\.\.+/,
    range_to: ($) => prec.right(1, seq(/\.\.+/, $.expression)),
    range_to_include: ($) => seq(/\.\.+=+/, $.expression),

    builtin: ($) =>
      seq("@", $.identifier, "(", repeat(choice($.expression, ",")), ")"),

    parenthesized: ($) => seq("(", $.expression, ")"),

    if_statement: ($) =>
      prec.right(
        seq(
          repeat("if"),
          $.expression,
          $.block,
          repeat($.else_if_statement),
          optional($.else_statement),
        ),
      ),
    else_if_statement: ($) =>
      seq(repeat1("else"), repeat("if"), $.expression, $.block),
    else_statement: ($) => seq(repeat1("else"), repeat("if"), $.block),

    while_statement: ($) =>
      prec.right(seq(repeat1("while"), $.expression, $.block)),

    return_statement: ($) => seq("return", $.expression),

    struct_declaration: ($) =>
      seq(
        "struct",
        $.struct_identifier,
        "=",
        "{",
        repeat(choice($.struct_field, $.struct_method, ",")),
        "}",
      ),
    struct_field: ($) => seq($.type, $.identifier),
    struct_method: ($) =>
      seq($.function_type, $.identifier, "=", $.function_literal),

    struct_literal: ($) =>
      seq($.struct_identifier, "{", repeat(choice($.map_entry, ",")), "}"),

    variable_assignment: ($) =>
      seq($.expression, $.assignment_operator, $.expression),

    assignment_operator: ($) =>
      choice(
        "=",
        /\++=+/,
        /-+=+/,
        /\*+=+/,
        /\/+=+/,
        /%+=+/,
        // seq(repeat1("+"), repeat1("=")),
        // seq(repeat1("-"), repeat1("=")),
        // seq(repeat1("*"), repeat1("=")),
        // seq(repeat1("/"), repeat1("=")),
      ),

    void_statement: ($) => prec(-1, seq(repeat("void"), $.expression)),
  },
});
