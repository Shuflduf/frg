/**
 * @file i like frogs fr
 * @author Shuflduf <shuflduf@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "frg",

  extras: ($) => [/\s/, $.comment],

  conflicts: ($) => [
    [$.set_literal, $.map_literal, $.empty_collection],
    [$.set_literal, $.map_literal],
    [$.type, $.expression],
    [$.type, $.struct_method],
    [$.statement, $.if_statement],
    [$.statement, $.block],
    [$.statement, $.set_literal],
    [$.statement, $.set_literal, $.if_statement],
    [$.statement, $.set_literal, $.block],
  ],

  rules: {
    source_file: ($) => repeat($.statement),

    statement: ($) =>
      choice(
        $.variable_declaration,
        $.if_statement,
        $.return_statement,
        $.struct_declaration,
        $.variable_assignment,
        $.expression,
      ),

    comment: ($) => token(seq("//", /.*/)),

    variable_declaration: ($) => seq($.type, $.identifier, "=", $.expression),

    type: ($) =>
      choice(
        "void",
        "int",
        "float",
        "str",
        "bool",
        "range",
        $.vec_type,
        $.set_type,
        $.map_type,
        $.function_type,
        $.identifier,
        $.reference_type,
      ),

    vec_type: ($) => seq("vec", "(", $.type, ")"),
    set_type: ($) => seq("set", "(", $.type, ")"),
    map_type: ($) => seq("map", "(", $.type, repeat(","), $.type, ")"),

    function_type: ($) => prec.right(seq($.type, repeat1($.parameter_list))),
    parameter_list: ($) => seq("(", repeat(choice($.type, ",")), ")"),

    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,
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
        $.set_literal,
        $.empty_collection,
        $.function_literal,
        $.function_call,
        $.dereference,
        $.member_access,
        $.index_access,
        $.unary_expression,
        $.range,
        $.builtin,
      ),

    binary_expression: ($) =>
      choice(
        prec.left(
          6,
          seq(
            $.expression,
            choice(">", "<", ">=", "<=", "==", "!=", "&&", "||"),
            $.expression,
          ),
        ),
        prec.left(11, seq($.expression, choice("+", "-"), $.expression)),
        prec.left(12, seq($.expression, choice("*", "/"), $.expression)),
        prec.left(5, seq($.expression, "&&", $.expression)),
      ),

    int_literal: ($) => /\d+/,
    float_literal: ($) => /\d+\.\d+/,
    string_literal: ($) => seq('"', /[^"]*/, '"'),
    bool_literal: ($) => choice("true", "false"),

    vec_literal: ($) => seq("[", repeat(choice($.expression, ",")), "]"),
    set_literal: ($) => seq("{", repeat(choice($.expression, ",")), "}"),
    map_literal: ($) => seq("{", repeat(choice($.map_entry, ",")), "}"),
    map_entry: ($) => seq($.expression, repeat1(":"), $.expression),
    empty_collection: ($) => prec(1, seq("{", repeat(","), "}")),

    function_literal: ($) => seq($.parameter_declaration, $.block),
    parameter_declaration: ($) =>
      seq("(", repeat(choice($.identifier, ",")), ")"),
    block: ($) => seq("{", repeat($.statement), optional($.expression), "}"),

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
      prec.left(11, seq($.expression, "[", $.expression, "]")),

    range: ($) =>
      prec.left(
        3,
        seq(
          optional($.expression),
          "..",
          choice(seq("=", $.expression), optional($.expression)),
          // optional("="),
          // optional($.expression),
        ),
      ),

    builtin: ($) =>
      seq("@", $.identifier, "(", repeat(choice($.expression, ",")), ")"),

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
      seq(repeat("else"), repeat("if"), $.expression, $.block),
    else_statement: ($) => seq(repeat("else"), repeat("if"), $.block),

    return_statement: ($) => seq("return", $.expression),

    struct_declaration: ($) =>
      seq(
        "struct",
        $.identifier,
        "=",
        "{",
        repeat(choice($.struct_field, $.struct_method, ",")),
        "}",
      ),
    struct_field: ($) => seq($.type, $.identifier),
    struct_method: ($) =>
      seq($.function_type, $.identifier, "=", $.function_literal),

    variable_assignment: ($) =>
      seq($.expression, choice("=", "+=", "-=", "*=", "/="), $.expression),
  },
});
