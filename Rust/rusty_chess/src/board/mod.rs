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
    player1_pieces: Vec<piece::Piece<'a>>,
    player2_pieces: Vec<piece::Piece<'a>>,
}

impl<'a> Board<'_>  {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for row in 0..self.grid_size {

            let mut space_row = Vec::<Space>::new();
            
            for col in 0..self.grid_size {

                let cur_space = 
                        Space::new((row as i32, col as i32), 
                        false, 
                        None, 
                            if (col + row) % 2 == 0 { color::Color::LIGHTGRAY} else { color::Color::GRAY });
                
                d.draw_rectangle(
                    cur_space.size * cur_space.position.0, 
                    cur_space.size * cur_space.position.1,
                    cur_space.size.into(), 
                    cur_space.size.into(), 
                    cur_space.c);
                space_row.push(cur_space);
            }
            self.spaces.push(space_row);
        }


    }
    
    pub fn new() -> Board<'a> {
        Board {
            grid_size: 8,
            spaces: Vec::<Vec::<Space>>::new(),
            player1_pieces: Vec::<piece::Piece<'a>>::new(),
            player2_pieces: Vec::<piece::Piece<'a>>::new(),
        }
    }
}