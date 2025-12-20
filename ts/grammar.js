/**
 * @file i like frogs fr
 * @author Shuflduf <shuflduf@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "frg",

  conflicts: ($) => [
    [$.set_literal, $.map_literal, $.empty_collection],
    [$.set_literal, $.map_literal],
  ],

  rules: {
    source_file: ($) => repeat($._statement),

    _statement: ($) => choice($.variable_declaration),

    variable_declaration: ($) => seq($.type, $.identifier, "=", $.expression),

    type: ($) =>
      choice("int", "float", "str", "bool", $.vec_type, $.set_type, $.map_type),

    vec_type: ($) => seq("vec", "(", $.type, ")"),
    set_type: ($) => seq("set", "(", $.type, ")"),
    map_type: ($) => seq("map", "(", $.type, repeat(","), $.type, ")"),

    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

    expression: ($) =>
      choice(
        $.binary_expression,
        $.number_literal,
        $.float_literal,
        $.string_literal,
        $.vec_literal,
        $.map_literal,
        $.set_literal,
        $.empty_collection,
      ),

    binary_expression: ($) =>
      choice(
        prec.left(1, seq($.expression, choice("+", "-"), $.expression)),
        prec.left(2, seq($.expression, choice("*", "/"), $.expression)),
      ),

    number_literal: ($) => /\d+/,
    float_literal: ($) => /\d+\.\d+/,
    string_literal: ($) => seq('"', /[^"]*/, '"'),

    vec_literal: ($) => seq("[", repeat(choice($.expression, ",")), "]"),
    set_literal: ($) => seq("{", repeat(choice($.expression, ",")), "}"),
    map_literal: ($) => seq("{", repeat(choice($.map_entry, ",")), "}"),

    empty_collection: ($) => seq("{", repeat(","), "}"),

    map_entry: ($) => seq($.expression, repeat1(":"), $.expression),
  },
});
