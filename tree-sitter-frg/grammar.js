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

  rules: {
    source_file: ($) => repeat($.statement),

    statement: ($) =>
      seq(choice($.if_statement, $.void_statement), repeat(";")),

    comment: ($) => token(seq("//", /.*/)),
    empty_collection: ($) => prec(-1, seq("{", repeat(","), "}")),

    expression: ($) => choice($.bool_literal, $.empty_collection),
    bool_literal: (_) => choice("true", "false"),
    block: ($) => seq("{", repeat($.statement), optional($.expression), "}"),

    if_statement: ($) =>
      prec.right(
        seq(
          repeat("if"),
          $.expression,
          $.block,
          optional(
            choice(
              seq(repeat1($.else_if_statement), optional($.else_statement)),
              prec(1, $.else_statement),
            ),
          ),
        ),
      ),
    else_if_statement: ($) =>
      seq(repeat("else"), repeat("if"), $.expression, $.block),
    else_statement: ($) => seq(repeat("else"), repeat("if"), $.block),

    void_statement: ($) => prec.right(-1, seq(repeat("void"), $.expression)),
  },
});
