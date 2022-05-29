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
#[derive(Debug, Copy, Clone)]
pub enum Movement{
    UP,
    DOWN,
    RIGHT,
    LEFT,
    NOP
}

#[wasm_bindgen]
pub struct Game {
    pub width: f64,
    pub height: f64,
    pub pos: Vec,
    pub speed: f64,
    pub movement: Movement 
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
            movement: Movement::NOP
        }
    }

    pub fn process(&mut self, _timespan: f64) {
        // here we have to mutate ourself
        match self.movement{
            Movement::DOWN => {
                self.pos.y += self.speed;
            }
            _ => {

            } 
        }


        // if self.pos.x > self.width {
        //     self.pos.x = -1.0;
        // }


        // if self.pos.y > self.height{
        //     self.pos.y = -1.0;
        // }


    }

    pub fn key_down(&mut self, event: web_sys::KeyboardEvent){
        let code = &event.code();
        if code == "KeyS" {
            self.movement = Movement::DOWN;
        }
    }

    pub fn key_up(&mut self, event: web_sys::KeyboardEvent){
    }
}
