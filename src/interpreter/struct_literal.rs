use std::collections::HashMap;

use chumsky::span::{SimpleSpan, Spanned};

use crate::{
    ast::Expr,
    errors::Diagnostic,
    interpreter::{self, CustomType, State, Value, Variable, get_type, var_type_check},
};

pub fn eval<'ast>(
    state: &State<'ast>,
    expr: &Box<Expr<'ast>>,
    fields: &Vec<(&'ast str, Expr<'ast>)>,
) -> Result<Value<'ast>, Diagnostic> {
    if let Ok(CustomType::StructType(vtype)) = get_type(&state, &expr) {
        println!("{vtype:?}");
        let mut struct_fields = HashMap::new();
        for (field_name, field_value) in fields {
            let field_value = interpreter::eval(state, &field_value)?;
            let field_type = vtype.iter().find(|t| t.1 == *field_name).unwrap().0.clone();
            if !var_type_check(&field_type, &field_value) {
                return Err(Diagnostic::new("shittter"));
            }
            let var = Variable {
                vtype_name: Spanned {
                    inner: "todo",
                    span: SimpleSpan::from(0..0),
                },
                vtype: field_type,
                value: field_value,
            };
            struct_fields.insert(field_name.to_string(), var);
        }
        return Ok(Value::Struct(struct_fields));
        // return Ok(Value::Struct())
    }
    Err(Diagnostic::new("this struct doesnt exist"))
}
