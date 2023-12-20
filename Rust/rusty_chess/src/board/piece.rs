use raylib::prelude::*;
use raylib::core::texture::Image;
use crate::board::Space;

#[derive(Clone, Copy)]
pub enum PieceName {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King, 
    Queen,
}

pub struct Piece {
    owner: bool,
    name: PieceName,
    piece_texture: Texture2D,
}

impl Piece {
    pub fn draw_self(self, d: &mut RaylibDrawHandle, rl: &mut RaylibHandle, thread: &RaylibThread) {
        d.draw_texture(&self.piece_texture, 300, 300, color::Color::WHITE);
    }

    pub fn new(owner: bool, name: PieceName, piece_texture: Texture2D) -> Piece {
        let pc = Piece {
            owner: owner,
            name: name,
            piece_texture: piece_texture,
        };
        pc
    }
}