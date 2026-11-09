#[allow(dead_code)]
pub enum VarType {
    Void,
    Int,
    Float,
    Str,
    Bool,
    StructDec,
}
#[allow(dead_code)]
pub enum Statement {
    VariableDeclaration { vtype: VarType },
}
