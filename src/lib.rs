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
    log("Console log from Rust WASM 🎯");
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> Vec<u32> {
    vec![a + b, a, b, a+b]
}

#[wasm_bindgen]
pub fn generate_image(width: u32, height: u32) -> Vec<u8> {
    let mut pixels = vec![0u8; (width * height * 4) as usize];
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            pixels[idx] = (x % 256) as u8;         // R
            pixels[idx + 1] = (y % 256) as u8;     // G
            pixels[idx + 2] = 128;                 // B
            pixels[idx + 3] = 255;                 // A
        }
    }
    pixels
}