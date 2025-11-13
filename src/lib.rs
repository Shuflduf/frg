use std::ffi::{CStr, CString, c_char};

pub mod ast;
pub mod interpreter;
pub mod lexer;

#[unsafe(no_mangle)]
pub extern "C" fn evaluate_frg(source_code: *const c_char) -> *mut c_char {
    let source_code = unsafe { CStr::from_ptr(source_code) }.to_str().unwrap();
    let tokens = lexer::lex(source_code);
    let ast = ast::parse(tokens);
    let result = interpreter::interpret(ast);

    let str_result = result
        .unwrap_or(interpreter::context::VariableValue::Void)
        .to_string();

    CString::new(str_result).unwrap().into_raw()
}
