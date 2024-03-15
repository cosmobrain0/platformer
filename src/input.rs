use std::collections::HashMap;

use ggez::winit::event::VirtualKeyCode;

#[derive(Debug)]
pub struct Keymap {
    keys: HashMap<VirtualKeyCode, bool>,
}
impl Keymap {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }
}
