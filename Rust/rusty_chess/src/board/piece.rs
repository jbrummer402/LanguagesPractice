use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use std::fs::File;
use std::io::{self, Read};
use std::fmt::Error;

#[derive(Debug, Eq, Hash, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Piece<'a> {
    pub piece_rect: (u8, u8),
    pub piece_type: PieceType,
    pub is_dragging: bool,
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


impl Piece<'_> {
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


    pub fn new(space: (u8,u8), piece_type: PieceType) -> Piece<'static> {
        // let mut rect = Rectangle{
        //     x: space.0 as f32 * 60.0,
        //     y: space.1 as f32 * 60.0,
        //     width: 60.0,
        //     height: 60.0
        // };

        Piece {
            piece_rect: space,
            piece_type: piece_type,
            is_dragging: false,
        }
    }
}
