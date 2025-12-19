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
    source_file: $ => repeat($._statement),

    _statement: $ => choice(
      $.variable_declaration
    ),

    variable_declaration: $ => seq(
      $.type,
      $.identifier,
      '=',
      $.expression
    ),

    type: $ => choice(
      'int',
      'float',
      'str',
      'bool'
    ),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    expression: $ => choice(
      $.number_literal
    ),

    number_literal: $ => /\d+/,
  }
});
