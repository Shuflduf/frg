pub use nodes::*;
use tree_sitter::{Tree, TreeCursor};

mod builtins;
mod expressions;
mod nodes;
mod statements;
mod types;

pub fn build(ts_tree: &Tree, code: &str) -> Vec<Statement> {
    let mut cursor = ts_tree.walk();
    parse_block(&mut cursor, code)
}

pub fn transpile(statements: &Vec<Statement>) -> String {
    let mut code = String::new();
    for statement in statements {
        let new_line = statements::transpile(statement);
        code.push_str(&new_line);
        code.push('\n');
    }
    code.pop();
    code
}

fn parse_block(cursor: &mut TreeCursor, code: &str) -> Vec<Statement> {
    let mut statements = vec![];
    cursor.goto_first_child();
    loop {
        let current_node_kind = cursor.node().kind();

        statements.push(match current_node_kind {
            "variable_declaration" | "if_statement" | "return_statement" | "struct_declaration" => {
                statements::parse(cursor, code, current_node_kind)
            }
            "function_call" | "builtin" | "int_literal" | "string_literal" | "float_literal"
            | "bool_literal" | "unary_expression" | "binary_expression" | "function_literal"
            | "index_access" | "map_literal" | "set_literal" => {
                Statement::Expression(expressions::parse(cursor, code))
            }
            "{" | "comment" => {
                cursor.goto_next_sibling();
                continue;
            }
            "}" => break,
            _ => todo!("{current_node_kind}"),
        });

        // println!("{current_node}");

        if !cursor.goto_next_sibling() {
            break;
        }
    }
    cursor.goto_parent();
    statements
}
