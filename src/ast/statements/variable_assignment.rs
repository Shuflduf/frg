use super::*;

type VarAss = (Expression, AssignmentOp, Expression);

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VarAss {
    cursor.goto_first_child();

    let left = expressions::parse(cursor, code);

    cursor.goto_next_sibling();
    let op = match &code[cursor.node().byte_range()] {
        "=" => AssignmentOp::Equals,
        "+=" => AssignmentOp::PlusEquals,
        _ => unreachable!(),
    };

    cursor.goto_next_sibling();
    let right = expressions::parse(cursor, code);

    cursor.goto_parent();
    (left, op, right)
}

pub fn transpile(var_ass: &VarAss) -> String {
    let left = expressions::transpile(&var_ass.0);
    let right = expressions::transpile(&var_ass.2);

    // if let VarType::Vec(_) = var_ass.
    let op = match &var_ass.1 {
        AssignmentOp::Equals => "=",
        AssignmentOp::PlusEquals => "+=",
        AssignmentOp::MinusEquals => "-=",
        AssignmentOp::TimesEquals => "*=",
        AssignmentOp::DivideEquals => "/=",
    };

    format!("{left} {op} {right};")
}
