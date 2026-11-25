use super::*;

pub fn eval<'ast>(
    state: &mut State<'ast>,
    span: SimpleSpan,
    parent: &Expr<'ast>,
    index: &Expr<'ast>,
) -> Result<Value<'ast>, Diagnostic> {
    let parent_value = deref_value(state, parent, span)?;
    let index_value = super::eval(state, index)?;
    let index_span = index.span;
    let index = match index_value {
        Value::Int(n) if n >= 0 => n as usize,
        other => {
            return Err(Diagnostic::basic(
                format!(
                    "index must be a non-negative int, found `{}`",
                    other.type_name()
                ),
                index_span,
            ));
        }
    };
    match parent_value {
        Value::List(values) => values
            .into_iter()
            .nth(index)
            .ok_or_else(|| Diagnostic::basic("index out of bounds", index_span)),
        other => Err(Diagnostic::basic(
            format!("cannot index into `{}`", other.type_name()),
            parent.span,
        )),
    }
}
