use raylib::prelude::*;
use raylib::core::texture::Image;
use crate::board::Space;

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
    fn get_image(self, file_path: String) -> Image {
        Image::load_image(&file_path).unwrap()
    }

    pub fn draw_self(rl: &mut RaylibHandle, thread: RaylibThread) {

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