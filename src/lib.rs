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

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

pub fn js_event_to_movement(event: web_sys::KeyboardEvent) -> Option<(Movement, Direction)> {
    let code = &event.code();

    if code == "KeyS" {
        Some((Movement::DOWN, Direction::Vertical))
    } else if code == "KeyW" {
        Some((Movement::UP, Direction::Vertical))
    } else if code == "KeyA" {
        Some((Movement::LEFT, Direction::Horizontal))
    } else if code == "KeyD" {
        Some((Movement::RIGHT, Direction::Horizontal))
    } else {
        None
    }
}

#[wasm_bindgen]
pub struct Game {
    pub width: f64,
    pub height: f64,
    pub pos: Vec,
    pub speed: f64,
    pub hmovement: Movement,
    pub vmovement: Movement,
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
            hmovement: Movement::NOP,
            vmovement: Movement::NOP,
        }
    }

    pub fn process(&mut self, _timespan: f64) {
        match self.hmovement{
            Movement::RIGHT => self.pos.x += self.speed,
            Movement::LEFT => self.pos.x -= self.speed,
            _ => (),
        }

        match self.vmovement{
            Movement::DOWN => self.pos.y += self.speed,
            Movement::UP => self.pos.y -= self.speed,
            _ => (),
        }
    }

    pub fn key_down(&mut self, event: web_sys::KeyboardEvent) {
        match js_event_to_movement(event){
            Some((m, Direction::Horizontal)) => self.hmovement = m,
            Some((m, Direction::Vertical)) => self.vmovement = m,
            None => (),
        }
    }

    pub fn key_up(&mut self, event: web_sys::KeyboardEvent) {
        match js_event_to_movement(event){
            Some((m, Direction::Horizontal)) => {
                if m == self.hmovement { 
                    self.hmovement = Movement::NOP
                }
            },
            Some((m, Direction::Vertical)) => {
                if m == self.vmovement { 
                    self.vmovement = Movement::NOP
                }
            },
            None => (),
        }
    }
}
