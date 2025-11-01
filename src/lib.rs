    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        // Import JavaScript's alert function
        fn alert(s: &str);
    }

    #[wasm_bindgen]
    pub fn greet(name: &str) {
        alert(&format!("Hello, {} from Rust WASM! 🙂", name));
    }

    #[wasm_bindgen]
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
    