mod utils;

use wasm_bindgen::prelude::*;
use web_sys;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
pub struct Game {
    pub width: f64,
    pub height: f64,
    pub pos: Vec,
    pub speed: f64,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Game {
        Game {
            width,
            height,
            pos: Vec { x: 0.0, y: 0.0 },
            speed: 0.1,
        }
    }

    pub fn process(&mut self, _timespan: f64) {
        // here we have to mutate ourself
        self.pos.x += self.speed;
        if self.pos.x > self.width {
            self.pos.x = -1.0;
        }
    }

    pub fn key_event(&mut self, event: web_sys::KeyboardEvent){
        log(&event.code())
    }
}
