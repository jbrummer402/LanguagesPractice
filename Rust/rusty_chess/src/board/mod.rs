pub mod piece;
use raylib::prelude::*;
use std::path::{PathBuf};
use glob::glob;

use substring::Substring;

use self::piece::Piece;

fn str_to_piecename(input: &str) -> Result<piece::PieceName, ()> {
    match input {
        "pawn" => Ok(piece::PieceName::Pawn),
        "knight" => Ok(piece::PieceName::Knight),
        "rook" => Ok(piece::PieceName::Rook),
        "bishop" => Ok(piece::PieceName::Bishop),
        "king" => Ok(piece::PieceName::King),
        "queen" => Ok(piece::PieceName::Queen),
        _ => Err(()),
    }
}

#[derive(Default)]
pub struct Space {
    position: (i32, i32),
    size: i32,
    taken: bool,
    c: color::Color,
}

impl Space {
    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.size * self.position.0, 
            self.size * self.position.1,
            self.size.into(), 
            self.size.into(), 
            self.c);
    }

    fn new(position: (i32, i32), taken: bool, c: color::Color) -> Space {
        Space {
            position: position,
            taken: taken,
            size: 45,
            c: c,
        }
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
    pub fn load_pieces(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut white_pieces: Vec<Piece> = Vec::new();
        let mut black_pieces: Vec<Piece> = Vec::new();

        for file in glob("./imgs/pieces-basic-png/*").expect("Directory not found") {
            
            match file {
                Ok(path) => {
                    let texture = rl.load_texture(thread, path.clone().into_os_string().to_str().expect("failed"));
                    
                    let path_substring: &str = &(path.clone().into_os_string().into_string().unwrap());
                    
                    let p = Piece::new(true, str_to_piecename(path_substring.substring(28, path_substring.len() - 4)).unwrap(), texture.expect("failed"));
                    
                    // if path.to_string_lossy().contains("white") {    
                    //     white_pieces.push(&p);
                    // } else {
                    //     black_pieces.push(&p);
                    // }
                    
                }
                
                _ => panic!("ruhroh")
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for row in 0..self.grid_size {
            for col in 0..self.grid_size {

                let mut cur_space = 
                        Space::new((row.into(), col.into()), 
                        false, 
                            if (col + row) % 2 == 0 { color::Color::LIGHTGRAY} else { color::Color::GRAY });
                
                cur_space.draw(d);
                self.spaces.push(cur_space);
            }
        }


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