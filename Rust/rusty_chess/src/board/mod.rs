pub mod piece;

use std::path::{PathBuf};

use raylib::prelude::*;
use glob::glob;


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
    fn new(position: (i32, i32), taken: bool, c: color::Color) -> Space {
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
            self.size * self.position.0, 
            self.size * self.position.1,
            self.size.into(), 
            self.size.into(), 
            self.c);
    }
}

#[derive(Default)]
pub struct Board {
    grid_size: u8,
    spaces: Vec<Space>,
    black_pieces: Vec<piece::Piece>,
    white_pieces: Vec<piece::Piece>,
}

impl Board {
    fn load_pieces(self) -> (Vec<PathBuf>, Vec<PathBuf>) {
        let mut white_pieces = Vec::<PathBuf>::new();
        let mut black_pieces = Vec::<PathBuf>::new();

        for file in glob("./imgs/pieces-basic-png/*").expect("Directory not found") {
            match file {
                Ok(path) => if String::from(path.to_string_lossy()).contains("white") {
                    white_pieces.push(path);
                } else {
                    black_pieces.push(path);
                }
                Err(e) => panic!("{:?}",e)
            }
        }

        (black_pieces, white_pieces)
    }
    
    pub fn new() -> Self {
        Board {
            grid_size: 8,
            spaces: Vec::<Space>::new(),
            black_pieces: Vec::<piece::Piece>::new(),
            white_pieces: Vec::<piece::Piece>::new(),
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