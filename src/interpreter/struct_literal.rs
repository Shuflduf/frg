use super::*;

pub fn eval<'ast>(
    state: &mut State<'ast>,
    parent: &Expr<'ast>,
    fields: &[(&'ast str, Expr<'ast>)],
) -> Result<Value<'ast>, Diagnostic> {
    let struct_fields = match super::eval(state, parent)? {
        Value::Type(TypeValue::Struct(fields)) => fields,
        other => {
            return Err(Diagnostic::basic(
                format!("expected a struct type, found `{}`", other.type_name()),
                parent.span,
            ));
        }
    };

    if fields.is_empty() {
        return Err(Diagnostic::basic(
            "struct literal needs at least one field",
            parent.span,
        ));
    }
    // if let Ok(CustomType::StructType(vtype)) = get_type(&state, &parent) {
    //     println!("{vtype:?}");
    let mut struct_values = HashMap::new();
    for (field_name, field_expr) in fields {
        let field_type = struct_fields
            .iter()
            .find(|(_, name)| name == field_name)
            .map(|(vtype, _)| vtype)
            .ok_or_else(|| {
                Diagnostic::basic(format!("unknown field `{}`", field_name), field_expr.span)
            })?;

        let field_value = super::eval(state, &field_expr)?;
        if !value_matches_type(&field_type, &field_value) {
            // TODO: make this diagnostic point to the struct defitinion
            return Err(Diagnostic::basic(
                format!(
                    "field `{}` expects `{}`, found `{}`",
                    field_name,
                    field_type_name(field_type),
                    field_value.type_name()
                ),
                field_expr.span,
            ));
        }
        struct_values.insert(
            field_name.to_string(),
            Variable {
                vtype_name: Spanned {
                    inner: *field_name,
                    span: field_expr.span,
                },
                vtype: field_type.clone(),
                value: field_value,
            },
        );
        // let var = Variable {
        //     vtype_name: Spanned {
        //         inner: "todo",
        //         span: SimpleSpan::from(0..0),
        //     },
        //     vtype: field_type,
        //     value: field_value,
        // };
    }

    Ok(Value::Struct(struct_values))
    //     return Ok(Value::Struct(struct_fields));
    //     // return Ok(Value::Struct())
    // }
    // Err(Diagnostic::new("this struct doesnt exist"))
}
fn field_type_name<'ast>(vtype: &TypeValue<'ast>) -> &'static str {
    match vtype {
        TypeValue::Primitive(VarType::Void) => "void",
        TypeValue::Primitive(VarType::Int) => "int",
        TypeValue::Primitive(VarType::Float) => "float",
        TypeValue::Primitive(VarType::Str) => "str",
        TypeValue::Primitive(VarType::Bool) => "bool",
        TypeValue::Primitive(VarType::Type) => "type",
        TypeValue::Primitive(VarType::List) => "list",
        TypeValue::Primitive(VarType::Custom(_)) => "custom",
        TypeValue::List(_) => "list",
        TypeValue::Struct(_) => "struct",
        TypeValue::Function { .. } => "function",
        TypeValue::Reference(_) => "reference",
    }
}
