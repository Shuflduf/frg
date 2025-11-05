use std::collections::HashMap;

use crate::ast::ast_nodes::*;

#[derive(Debug)]
enum VariableValue {
    Int(i32),
}

pub fn interpret(ast: Vec<Statement>) {
    let mut declared_variables: HashMap<String, VariableValue> = Default::default();
    for statement in ast {
        match statement {
            Statement::VariableDeclaration {
                var_type,
                name,
                value,
            } => declare_variable(&mut declared_variables, var_type, name, value),
            _ => todo!(),
        }
    }
    dbg!(declared_variables);
}

fn declare_variable(
    declared_variables: &mut HashMap<String, VariableValue>,
    _var_type: VarType,
    name: String,
    value: Expression,
) {
    declared_variables.insert(name, eval(value));
}

fn eval(expression: Expression) -> VariableValue {
    match expression {
        Expression::Literal(literal) => match literal {
            Literal::Int(int) => VariableValue::Int(int),
            _ => todo!(),
        },
        _ => todo!(),
    }
}
