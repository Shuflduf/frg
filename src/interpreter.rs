use std::{collections::HashMap, ops::Add};

use crate::ast::{BinaryOperator, Expression, Literal, Statement, VarType};

#[derive(Debug)]
enum Value {
    Int(i64),
    Float(f64),
}

impl Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Int(lnum) => match rhs {
                Self::Int(rnum) => Self::Int(lnum + rnum),
                Self::Float(rnum) => Self::Float(lnum as f64 + rnum),
            },
            Self::Float(lnum) => match rhs {
                Self::Float(rnum) => Self::Float(lnum + rnum),
                Self::Int(rnum) => Self::Float(lnum + rnum as f64),
            },
        }
    }
}

#[derive(Debug)]
struct State<'src> {
    variables: HashMap<&'src str, Value>,
}

pub fn run(program: &[Statement]) -> Result<i64, String> {
    let mut statements = program.iter();
    let mut state = State {
        variables: HashMap::new(),
    };
    while let Some(stmt) = statements.next() {
        match stmt {
            Statement::VariableDeclaration { vtype, name, value } => {
                let val = eval(&state, value);
                let correct_type = match val {
                    Value::Int(_) => vtype == &VarType::Int,
                    Value::Float(_) => vtype == &VarType::Float,
                };
                assert!(correct_type);
                state.variables.insert(*name, val);
            }
        }
    }

    println!("{state:?}");

    Ok(0)
}

fn eval(state: &State, expr: &Expression) -> Value {
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::Int(num) => Value::Int(*num),
            Literal::Float(num) => Value::Float(*num),
            _ => todo!(),
        },
        Expression::BinaryOperation { left, op, right } => match op {
            BinaryOperator::Add => eval(state, left) + eval(state, right),
            _ => todo!(),
        },
        _ => todo!(),
    }
}
