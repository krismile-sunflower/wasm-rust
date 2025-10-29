
### Hello World

#### 初始化
cargo new --lib hello_world

#### Cargo.toml
``` toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"] # 创建动态链接库 （C-compatible dynamic library ）

[dependencies]
wasm-bindgen = "0.2.105"

```

#### src/lib.rs
``` rust
#[wasm_bindgen]
extern "C" { // extern 块用于声明外部函数
    // 绑定 JavaScript 的 alert 函数
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

#### 打包

方案1
- cargo build --target wasm32-unknown-unknown
- wasm-bindgen target/wasm32-unknown-unknown/debug/hello_world.wasm --out-dir ../vite-project/src/assets/pkg

方案2
- wasm-pack build --out-dir ../vite-project/src/assets/pkg

### vite-project

#### vite.config.ts
配置plugin插件
``` ts
pnpm add - D vite-plugin-wasm
```

#### 使用
``` ts
const rust = import('./assets/pkg/hello_world.js');
rust
    .then(m => m.hello("Vite + React + Wasm"))
    .catch(console.error);
// 项目实际效果弹框展示：Hello Vite + React + Wasm
```
