use super::*;

type StructDecl = (String, Vec<(VarType, String)>);

pub fn parse(cursor: &mut TreeCursor, code: &str) -> StructDecl {
    cursor.goto_first_child();

    // skip "struct"
    cursor.goto_next_sibling();
    let struct_name = code[cursor.node().byte_range()].to_string();

    // skip "="
    cursor.goto_next_sibling();
    let fields = get_fields(cursor, code);

    cursor.goto_parent();
    (struct_name, fields)
}

pub fn transpile(struct_decl: &StructDecl) -> String {
    let struct_name = &struct_decl.0;
    let mut fields_str = String::new();
    struct_decl
        .1
        .iter()
        .map(|field| format!("{}: {},\n", field.1, types::transpile(&field.0)))
        .for_each(|field| fields_str.push_str(&field));
    fields_str.pop();
    format!("#[derive(Clone)]\nstruct {struct_name} {{\n{fields_str}\n}}")
}

fn get_fields(cursor: &mut TreeCursor, code: &str) -> Vec<(VarType, String)> {
    let mut fields = vec![];
    while cursor.goto_next_sibling() {
        let node_kind = cursor.node().kind();
        fields.push(match node_kind {
            "{" | "," => continue,
            "}" => break,
            "struct_field" => {
                cursor.goto_first_child();
                let field_type = types::parse(cursor, code);

                cursor.goto_next_sibling();
                let field_name = code[cursor.node().byte_range()].to_string();

                cursor.goto_parent();
                (field_type, field_name)
            }
            _ => todo!(),
        })
    }
    fields
}
