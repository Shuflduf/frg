use super::*;

pub fn eval<'ast>(
    state: &mut State<'ast>,
    span: SimpleSpan,
    parent: &Expr<'ast>,
    member: Spanned<&'ast str>,
) -> Result<Value<'ast>, Diagnostic> {
    if matches!(&parent.inner, &STD) && member.inner == "debug" {
        return Ok(Value::Int(1));
    }
    match deref_value(state, parent, span)? {
        Value::Struct(vars) => vars
            .get(member.inner)
            .map(Variable::value)
            .ok_or_else(|| Diagnostic::basic(format!("no field `{}`", member.inner), member.span)),
        other => Err(Diagnostic::basic(
            format!(
                "cannot access the field `{}` on `{}`",
                member.inner,
                other.type_name()
            ),
            member.span,
        )),
    }
}
