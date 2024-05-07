#![deny(clippy::all)]
#[macro_use]
extern crate napi_derive;
use mouse_position::mouse_position::Mouse;
#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
#[napi(object)]
pub struct MousePosition {
  pub x: i32,
  pub y: i32,
}
#[napi]
pub fn get_mouse_position() -> MousePosition {
  let position: Mouse = Mouse::get_mouse_position();
  match position {
    Mouse::Position { x, y } => MousePosition { x, y },
    Mouse::Error => MousePosition { x: 0, y: 0 }
  } 
}