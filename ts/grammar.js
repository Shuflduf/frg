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
  ],

  rules: {
    source_file: ($) => repeat($._statement),

    _statement: ($) => choice($.variable_declaration, $.if_statement),

    comment: ($) => token(seq("//", /. */)),

    variable_declaration: ($) => seq($.type, $.identifier, "=", $.expression),

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
      ),

    vec_type: ($) => seq("vec", "(", $.type, ")"),
    set_type: ($) => seq("set", "(", $.type, ")"),
    map_type: ($) => seq("map", "(", $.type, repeat(","), $.type, ")"),

    function_type: ($) => prec.right(seq($.type, repeat1($.parameter_list))),
    parameter_list: ($) => seq("(", repeat(choice($.type, ",")), ")"),

    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

    expression: ($) =>
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
      ),

    binary_expression: ($) =>
      choice(
        prec.left(
          0,
          seq(
            $.expression,
            choice(">", "<", ">=", "<=", "==", "!="),
            $.expression,
          ),
        ),
        prec.left(1, seq($.expression, choice("+", "-"), $.expression)),
        prec.left(2, seq($.expression, choice("*", "/"), $.expression)),
      ),

    number_literal: ($) => /\d+/,
    float_literal: ($) => /\d+\.\d+/,
    string_literal: ($) => seq('"', /[^"]*/, '"'),
    bool_literal: ($) => choice("true", "false"),

    vec_literal: ($) => seq("[", repeat(choice($.expression, ",")), "]"),
    set_literal: ($) => seq("{", repeat(choice($.expression, ",")), "}"),
    map_literal: ($) => seq("{", repeat(choice($.map_entry, ",")), "}"),
    map_entry: ($) => seq($.expression, repeat1(":"), $.expression),
    empty_collection: ($) => seq("{", repeat(","), "}"),

    function_literal: ($) => seq($.parameter_declaration, $.block),
    parameter_declaration: ($) =>
      seq("(", repeat(choice($.identifier, ",")), ")"),
    block: ($) => seq("{", repeat($._statement), optional($.expression), "}"),

    // it would theoretically be possible to switch $.identifier for $.expression so you can do shit like 5()
    function_call: ($) =>
      prec(10, seq($.identifier, "(", repeat(choice($.expression, ",")), ")")),

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
  },
});
