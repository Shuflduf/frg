use super::{Expression, AssignmentOp, TreeCursor, expressions, skip_repeats};

type VarAss = (Expression, AssignmentOp, Expression);

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VarAss {
    cursor.goto_first_child();

    let left = expressions::parse(cursor, code);

    cursor.goto_next_sibling();
    let op = match &code.chars().nth(cursor.node().byte_range().start).unwrap() {
        '=' => AssignmentOp::Equals,
        '+' => {
            skip_repeats(cursor, code, "+");
            AssignmentOp::PlusEquals
        }
        '-' => {
            skip_repeats(cursor, code, "-");
            AssignmentOp::MinusEquals
        }
        '*' => {
            skip_repeats(cursor, code, "*");
            AssignmentOp::TimesEquals
        }
        '/' => {
            skip_repeats(cursor, code, "/");
            AssignmentOp::DivideEquals
        }
        _ => unreachable!("{}", &code[cursor.node().byte_range()]),
    };

    skip_repeats(cursor, code, "=");

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
