    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        fn alert(s: &str); // imporitng js alert function

        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    #[wasm_bindgen]
    pub fn greet(name: &str) {
        alert(&format!("Hello, {} from Rust WASM! 🙂", name));
    }

    #[wasm_bindgen]
    pub fn add(a: u32, b: u32) -> Vec<u32> {
        vec![a + b, a, b, a+b]
    }
    