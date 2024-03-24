use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use std::fs::File;
use std::io::{self, Read};

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
    pub piece_rect: Rectangle,
    pub piece_type: PieceType,
    pub is_dragging: bool,
    pub space: Rectangle
}

// trait Snap {
//     // add code here
//     fn snap_to_space(&self, space: Rectangle);
// }
//
// impl Snap for Piece {
//     // add code here
//     //
//     fn snap_to_space(&self, space: Rectangle) -> Result<(), Error> {
//         
//         
//
//     }
// }
// trait Movement {
//     
//     fn move_pawn(piece_selected: (u8, u8), layout: &mut [[u8; 8]; 4] );
//
// }
//
// impl Movement for Piece {
//         
//     fn move_pawn(piece_selected: (u8, u8), layout: &mut [[u8; 8]; 4] ) -> Result<(),io::Error>{
//         let row = piece_selected.0;
//         let col = piece_selected.1;
//
//         let moving_first = if row == 2 || row == 3 { true } else { false };
//     
//
//     }
//
//     
// }


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


    pub fn new(piece_rect: Rectangle, piece_type: PieceType, row: f32, col: f32, size: f32) -> Piece {

        let mut rect = Rectangle {
                        x: col * size,
                        y: row * size,
                        width: size, 
                        height: size,
                    };
        Piece {
            piece_rect: rect,
            piece_type: piece_type,
            is_dragging: false,
            space: rect,
        }
    }
}
