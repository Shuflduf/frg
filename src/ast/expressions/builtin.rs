use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> BuiltinCall {
    cursor.goto_first_child();
    // skip "@"
    cursor.goto_next_sibling();

    let name = code[cursor.node().byte_range()].to_string();

    cursor.goto_next_sibling();
    let params = function_call::parameters(cursor, code);

    cursor.goto_parent();
    BuiltinCall { name, params }
}

pub fn transpile(builtin_call: &BuiltinCall) -> String {
    match &builtin_call.name[..] {
        "print" => builtins::print::transpile(&builtin_call.params),
        _ => panic!("invalid builtin {}", builtin_call.name),
    }
}
