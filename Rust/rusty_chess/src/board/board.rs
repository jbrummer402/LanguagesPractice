use raylib::prelude::*;

pub trait Drawable<T> {
    fn draw(&mut self, d: &mut RaylibDrawHandle);
}

#[derive(Default)]
pub struct Space {
    position: (i32, i32),
    size: i32,
    taken: bool,
    c: color::Color,
}

impl Space {
    pub fn new(position: (i32, i32), taken: bool, c: color::Color) -> Self {
        Space {
            position: position,
            taken: taken,
            size: 45,
            c: c,
        }
    }
}

impl Drawable<Space> for Space {
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            (self.size * self.position.0), 
            (self.size * self.position.1),
            (self.size).into(), 
            (self.size).into(), 
            self.c);
    }
}

#[derive(Default)]
pub struct Board {
    grid_size: u8,
    spaces: Vec<Space>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid_size: 8,
            spaces: Vec::<Space>::new(),
        }
    }
}

impl Drawable<Board> for Board {
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for row in 0..self.grid_size {
            for col in 0..self.grid_size {

                let mut cur_space = Space::new((row.into(), col.into()), 
                                                    false, 
                                                        if (col + row) % 2 == 0 { color::Color::LIGHTGRAY} else { color::Color::GRAY });
                cur_space.draw(d);
                self.spaces.push(cur_space);
            }
        }
    }
}

// impl Board {
//     pub fn draw_board(&self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        
//     }
// }