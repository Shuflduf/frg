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
    [$.set_literal, $.map_literal, $.empty_collection, $.block],
    [$.set_literal, $.if_statement],
    [$.set_literal, $.block],
    [$.type, $._expression],
    [$.type, $.struct_method],
  ],

  rules: {
    source_file: ($) => repeat($._statement),

    _statement: ($) =>
      choice(
        $.variable_declaration,
        $.if_statement,
        $.return_statement,
        $.struct_declaration,
        $.variable_assignment,
      ),

    comment: ($) => token(seq("//", /.*/)),

    variable_declaration: ($) => seq($.type, $.identifier, "=", $._expression),

    type: ($) =>
      choice(
        "void",
        "int",
        "float",
        "str",
        "bool",
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

    reference_type: ($) => prec(1, seq("&", $.type)),

    _expression: ($) =>
      choice(
        $.identifier,
        $.binary_expression,
        $.number_literal,
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
      ),

    binary_expression: ($) =>
      choice(
        prec.left(
          6,
          seq(
            $._expression,
            choice(">", "<", ">=", "<=", "==", "!="),
            $._expression,
          ),
        ),
        prec.left(11, seq($._expression, choice("+", "-"), $._expression)),
        prec.left(12, seq($._expression, choice("*", "/"), $._expression)),
        prec.left(5, seq($._expression, "&&", $._expression)),
      ),

    number_literal: ($) => /\d+/,
    float_literal: ($) => /\d+\.\d+/,
    string_literal: ($) => seq('"', /[^"]*/, '"'),
    bool_literal: ($) => choice("true", "false"),

    vec_literal: ($) => seq("[", repeat(choice($._expression, ",")), "]"),
    set_literal: ($) => seq("{", repeat(choice($._expression, ",")), "}"),
    map_literal: ($) => seq("{", repeat(choice($.map_entry, ",")), "}"),
    map_entry: ($) => seq($._expression, repeat1(":"), $._expression),
    empty_collection: ($) => seq("{", repeat(","), "}"),

    function_literal: ($) => seq($.parameter_declaration, $.block),
    parameter_declaration: ($) =>
      seq("(", repeat(choice($.identifier, ",")), ")"),
    block: ($) => seq("{", repeat($._statement), optional($._expression), "}"),

    // it would theoretically be possible to switch $.identifier for $._expression so you can do shit like 5()
    function_call: ($) =>
      prec(15, seq($.identifier, "(", repeat(choice($._expression, ",")), ")")),

    unary_expression: ($) =>
      choice(
        prec(14, seq("&", $._expression)),
        prec(13, seq("-", $._expression)),
        prec(13, seq("!", $._expression)),
      ),
    // reference: ($) => prec.right(14, ),

    dereference: ($) => prec.left(16, seq($._expression, ".*")),
    member_access: ($) => prec.left(16, seq($._expression, ".", $.identifier)),

    index_access: ($) =>
      prec.left(11, seq($._expression, "[", $._expression, "]")),

    range: ($) =>
      prec.left(
        3,
        seq(
          optional($._expression),
          "..",
          optional("="),
          optional($._expression),
        ),
      ),

    if_statement: ($) =>
      prec.right(
        seq(
          repeat("if"),
          $._expression,
          $.block,
          repeat($.else_if_statement),
          optional($.else_statement),
        ),
      ),
    else_if_statement: ($) =>
      seq(repeat("else"), repeat("if"), $._expression, $.block),
    else_statement: ($) => seq(repeat("else"), repeat("if"), $.block),

    return_statement: ($) => seq("return", $._expression),

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
      seq($._expression, choice("=", "+=", "-=", "*=", "/="), $._expression),
  },
});
