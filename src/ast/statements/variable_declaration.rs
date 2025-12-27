use tree_sitter::TreeCursor;

// use crate::ast_builder::{expressions, types, VariableDeclaration};
use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VariableDeclaration {
    // enter variable delcaration
    cursor.goto_first_child();
    let var_type = types::parse(cursor, code);

    cursor.goto_next_sibling();
    let identifier = code[cursor.node().byte_range()].to_string();

    // skip "="
    cursor.goto_next_sibling();
    cursor.goto_next_sibling();

    println!("2: {}", &code[cursor.node().byte_range()]);
    let value = expressions::parse(cursor, code);

    cursor.goto_parent();
    println!("3: {}", &code[cursor.node().byte_range()]);

    VariableDeclaration {
        var_type,
        identifier,
        value,
    }
}

pub fn transpile(var_decl: &VariableDeclaration) -> String {
    let var_name = &var_decl.identifier;
    let value = expressions::transpile(&var_decl.value);
    let type_str = types::transpile(&var_decl.var_type);
    let value = if let VarType::Struct(struct_name) = &var_decl.var_type
        && let Expression::MapLiteral(map_lit) = &var_decl.value
    {
        let struct_fields = expressions::map_literal::transpile_struct(map_lit);
        format!("{struct_name} {struct_fields}")
    } else if let Expression::EmptyCollection = &var_decl.value {
        match &var_decl.var_type {
            VarType::Struct(struct_name) => format!("{struct_name} {{}}"),
            VarType::Set(_) => "std::collections::HashSet::new()".into(),
            VarType::Map(_) => "std::collections::HashMap::new()".into(),
            _ => panic!("wtf you cant assign that :(("),
        }
    } else {
        value
    };
    format!("let mut {var_name}: {type_str} = {value};")
}
