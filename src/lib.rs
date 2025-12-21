// use wasm_bindgen::prelude::wasm_bindgen;

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;

// #[wasm_bindgen]
// pub fn evaluate_frg(source_code: String) -> String {
//     let tokens = lexer::lex(&source_code);
//     let ast = ast::parse(tokens);
//     let result = interpreter::interpret(ast);

//     result
//         .unwrap_or(interpreter::context::VariableValue::Void)
//         .to_string()
// }
