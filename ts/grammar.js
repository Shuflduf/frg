/**
 * @file i like frogs fr
 * @author Shuflduf <shuflduf@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "frg",

  rules: {
    source_file: ($) => repeat($._statement),

    _statement: ($) => choice($.variable_declaration),

    variable_declaration: ($) => seq($.type, $.identifier, "=", $.expression),

    type: ($) => choice("int", "float", "str", "bool", $.vec_type),

    vec_type: ($) => seq("vec", "(", $.type, ")"),

    identifier: ($) => /[a-zA-Z_][a-zA-Z0-9_]*/,

    expression: ($) =>
      choice(
        $.binary_expression,
        $.number_literal,
        $.float_literal,
        $.string_literal,
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
  },
});
