mod utils;

#[macro_use]
extern crate fstrings;


use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Game {
    pub width: i32,
    pub height: i32,
}


#[wasm_bindgen]
impl Game{
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Game{
        Game{
            width,
            height
        }   
    }


    pub fn process(&mut self, timespan: f64){}

}
