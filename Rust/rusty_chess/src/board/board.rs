use raylib::prelude::*;

pub struct Space {
    x_pos: u8,
    y_pos: u8,
    width: u8,
    height: u8,
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
                d.draw_rectangle(20 * row, 20 * col, 15, 15, if (col + row) % 2 == 0 { Color::BLACK} else {Color::GRAY });
            }
        }
    }
}