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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Movement {
    UP,
    DOWN,
    RIGHT,
    LEFT,
    NOP,
}


pub fn js_event_to_movement(event: web_sys::KeyboardEvent) -> Movement{
        let code = &event.code();

        if code == "KeyS" {
              Movement::DOWN
        }else if code == "KeyW"{
              Movement::UP
        }else if code == "KeyA"{
             Movement::LEFT
        }else if code == "KeyD"{
            Movement::RIGHT
        }else{
            Movement::NOP
        }
}

#[wasm_bindgen]
pub struct Game {
    pub width: f64,
    pub height: f64,
    pub pos: Vec,
    pub speed: f64,
    pub movement: Movement,
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
            movement: Movement::NOP,
        }
    }

    pub fn process(&mut self, _timespan: f64) {
        match self.movement {
            Movement::DOWN => self.pos.y += self.speed,
            Movement::UP => self.pos.y -= self.speed,
            Movement::RIGHT => self.pos.x += self.speed,
            Movement::LEFT => self.pos.x -= self.speed,
            Movement::NOP => (),
        }

    }

    pub fn key_down(&mut self, event: web_sys::KeyboardEvent) {
        self.movement = js_event_to_movement(event)
    }

    pub fn key_up(&mut self, event: web_sys::KeyboardEvent) {
        let movement = js_event_to_movement(event);
        if movement == self.movement {
            self.movement = Movement::NOP
        }

    }
}
