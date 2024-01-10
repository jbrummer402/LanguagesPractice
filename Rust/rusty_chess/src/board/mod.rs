pub mod piece;
use raylib::prelude::*;
use crate::board::piece::Piece;

#[derive(Default)]
pub struct Space {
    pub position: (i32, i32),
    size: i32,
    taken: bool,
    piece: Option<Piece>,
    c: color::Color,
}

impl Space {
    fn new(position: (i32, i32), taken: bool, piece: Option<Piece>, c: color::Color) -> Space {
        Space {
            position: position,
            taken: taken,
            size: 60,
            piece: piece,
            c: c,
        }
    }
}

#[derive(Default)]
pub struct Board {
    pub grid_size: u8,
    pub spaces: Vec<Vec<Space>>,
}

impl Board  {    
    pub fn new() -> Board {
        Board {
            grid_size: 8,
            spaces: Vec::<Vec::<Space>>::new(),
        }
    }
}
