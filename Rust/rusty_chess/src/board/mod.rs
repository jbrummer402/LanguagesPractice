pub mod piece;
use raylib::prelude::*;


#[derive(Default)]
pub struct Space<'a> {
    pub position: (i32, i32),
    size: i32,
    taken: bool,
    piece: Option<piece::Piece<'a>>,
    c: color::Color,
    
}

impl<'a> Space<'_> {
    fn new(position: (i32, i32), taken: bool, piece: Option<piece::Piece>, c: color::Color) -> Space {
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
pub struct Board<'a> {
    pub grid_size: u8,
    pub spaces: Vec<Vec<Space<'a>>>,
}

impl<'a> Board<'_>  {    
    pub fn new() -> Board<'a> {
        Board {
            grid_size: 8,
            spaces: Vec::<Vec::<Space>>::new(),
        }
    }
}