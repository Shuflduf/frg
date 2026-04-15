use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> BinaryOperation {
    cursor.goto_first_child();
    let left = Box::new(expressions::parse(cursor, code));

    cursor.goto_next_sibling();
    let op_symbol = &code[cursor.node().byte_range()];
    let op = match op_symbol.chars().nth(0).unwrap() {
        '+' => {
            skip_repeats(cursor, code, "+");
            BinaryOperator::Add
        }
        '-' => {
            skip_repeats(cursor, code, "-");
            BinaryOperator::Subtract
        }
        '*' => {
            skip_repeats(cursor, code, "*");
            BinaryOperator::Multiply
        }
        '/' => {
            skip_repeats(cursor, code, "/");
            BinaryOperator::Divide
        }
        '=' => {
            skip_repeats(cursor, code, "=");
            BinaryOperator::Equals
        }
        '!' => {
            skip_repeats(cursor, code, "!");
            BinaryOperator::NotEquals
        }
        '<' => {
            skip_repeats(cursor, code, "<");
            if op_symbol.chars().nth_back(0).unwrap() == '=' {
                skip_repeats(cursor, code, "=");
                BinaryOperator::LessThanOrEqual
            } else {
                BinaryOperator::LessThan
            }
        }
        '>' => {
            skip_repeats(cursor, code, ">");
            if op_symbol.chars().nth_back(0).unwrap() == '=' {
                skip_repeats(cursor, code, "=");
                BinaryOperator::GreaterThanOrEqual
            } else {
                BinaryOperator::GreaterThan
            }
        }
        _ => unreachable!("{}", op_symbol),
    };

    cursor.goto_next_sibling();
    let right = Box::new(expressions::parse(cursor, code));
    // let op = match cursor.nod

    cursor.goto_parent();
    BinaryOperation { left, op, right }
    // todo!()
}

pub fn transpile(binary_op: &BinaryOperation) -> String {
    let left = expressions::transpile(&binary_op.left);
    let right = expressions::transpile(&binary_op.right);
    let op = match binary_op.op {
        BinaryOperator::Add => "+",
        BinaryOperator::Subtract => "-",
        BinaryOperator::Multiply => "*",
        BinaryOperator::Divide => "/",
        BinaryOperator::LessThan => "<",
        BinaryOperator::LessThanOrEqual => "<=",
        BinaryOperator::GreaterThan => ">",
        BinaryOperator::GreaterThanOrEqual => ">=",
        BinaryOperator::Equals => "==",
        BinaryOperator::NotEquals => "!=",
    };
    format!("{left} {op} {right}")
}
