use raylib::prelude::*;
use raylib::core::texture::Image;
use crate::board::Space;

#[derive(Clone, Copy, PartialEq, Hash, Eq, Debug)]
pub enum PieceName {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King, 
    Queen,
}


trait Behvaior {
    fn can_move(&self);
}

impl Behvaior for PieceName {
    fn can_move(&self) {
        match self {
            PieceName::Pawn => println!("Pawn has moved"),
            PieceName::Rook => println!("Rook has moved"),
            PieceName::Bishop => println!("Bishop has moved"),
            PieceName::Knight => println!(),
            PieceName::King => println!(),
            PieceName::Queen => println!(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece<'a> {
    pub owner: bool,
    pub name: PieceName,
    pub piece_texture: &'a Texture2D,
    pub position: (i8, i8),
}

trait Movement {
    fn can_move(self);
}

impl Movement for PieceName {
    fn can_move(self) {
        
    }
}

impl<'a> Piece<'_> {
    pub fn draw_self(self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.piece_texture, self.position.0.into(), self.position.1.into(), color::Color::WHITE);
    }

    pub fn new(owner: bool, name: PieceName, piece_texture: &'a Texture2D, position: (i8, i8)) -> Piece<'a> {
        Piece {
            owner: owner,
            name: name,
            piece_texture: piece_texture,
            position: position, 
        }
    }
}