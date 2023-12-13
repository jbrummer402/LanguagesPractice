use raylib::prelude::*;

pub struct Space {
    x_pos: u8,
    y_pos: u8,
    taken: bool,
}

#[derive(Default)]
pub struct Board {
    spaces: Vec<Space>,
}

impl Board {
    fn default() -> Self {
        return Board {
            spaces: Vec::new(),
        }
    }
    pub fn draw(&self, mut d: RaylibDrawHandle, thread: &RaylibThread) {
        for row in 0..8 {
            for col in 0..8 { 
                d.draw_rectangle(20 * row, 20 * col, 10, 10, Color::BLACK);
            }
        }
    }
}