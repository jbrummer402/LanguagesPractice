use raylib::consts::MouseButton::*;
use raylib::prelude::*;

#[derive(Debug, Eq, Hash, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Piece {
    pub rect: Rectangle,
    pub r#type: PieceType,
    pub is_dragging: bool,
}

impl Piece {
    pub fn piece_to_name(index: u8) -> PieceType {
        match index {
            1 => PieceType::Pawn,
            2 => PieceType::Rook,
            3 => PieceType::Knight,
            4 => PieceType::Bishop,
            5 => PieceType::Queen,
            6 => PieceType::King,
            _ => PieceType::Pawn,
        }
    }

    pub fn new(rect: Rectangle, piece_type: PieceType, row: f32, col: f32, size: f32) -> Piece {

        let mut rect = Rectangle {
                        x: col * size,
                        y: row * size,
                        width: size, 
                        height: size,
                    };
        Piece {
            rect: rect,
            r#type: piece_type,
            is_dragging: false,
        }
    }
}
