extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use uuid::Uuid;

#[wasm_bindgen]
extern "C" { // extern 块用于声明外部函数
    // 绑定 JavaScript 的 alert 函数
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}


#[wasm_bindgen]
pub fn uuid() -> JsValue {
    let id = Uuid::new_v4();
    id.to_string().into()
}