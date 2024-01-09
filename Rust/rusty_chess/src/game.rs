use raylib::prelude::*;
use crate::board::{Board, piece};
use crate::board::piece::{Piece, PieceName};
use std::collections::HashMap;

use std::{panic, u8};
use std::fmt::Error;
use glob::glob;

fn string_to_piece_name(s: &str, te: Texture2D) -> Result::<PieceName, Error> {
    match s {
        "pawn" => Ok(PieceName::Pawn(te)),
        "rook" => Ok(PieceName::Rook(te)),
        "bishop" => Ok(PieceName::Bishop(te)),
        "knight" => Ok(PieceName::Knight(te)),
        "king" => Ok(PieceName::King(te)),
        "queen" => Ok(PieceName::Queen(te)),
        _ => panic!("bitch"),
    }
}

pub struct Game<'a> {
    pub board: Board<'a>,
    pub turn: bool,
    pub layout: [[u8; 8]; 8],
    piece_textures: HashMap<u8, Vec<PieceName>>,
    // Map the names of pieces to the number of them left
    // per player
    // pub player1_pieces: HashMap<PieceName, u8>,
    // pub player2_pieces: HashMap<PieceName, u8>,
    // white_textures: HashMap<&'a Piece<'a>, Texture2D>,
    // black_textures: HashMap<&'a Piece<'a>, Texture2D>,
}

impl<'a> Game<'_> {
    pub fn draw_board(&mut self, d: &mut RaylibDrawHandle) {
        for row in 0..8 {
            for col in 0..8 {
                d.draw_rectangle(
                    60 * row, 
                    60 * col,
                    60, 
                    60, 
                    if (col + row) % 2 == 0 { color::Color::LIGHTGRAY} else { color::Color::GRAY });
            }
        }

    }
    // 
    pub fn draw_pieces(&self, d: &mut RaylibDrawHandle, 
                       p_text: &Texture2D, 
                       position: (u16, u16), 
                       space_between: u16, 
                       num_pieces: u8) {
        
        // Iterate through every piece currently on the board
        for row in 0..self.layout.len() {
            for col in self.layout[row] {
                match col {
                    8 => println!(),
                    _ => println!("Not 8"),
                    
                }

                // match name.unwrap() {
                //     PieceName::Pawn => {
                        
                        
                        
                //     },
                //     PieceName::Rook => {},
                //     PieceName::Knight => {},
                //     PieceName::Bishop => {},
                //     PieceName::Queen => {},
                //     PieceName::King => {},
                // }
            }
        }

        let mut piece_rects = Vec::<Rectangle>::new();

        for i in 0..num_pieces {
            let left: Rectangle = Rectangle::new(position.0 as f32,  position.1 as f32, 60.0, 60.0);

            piece_rects.push(left);
            let right: Rectangle = Rectangle::new((position.0 + 60 * space_between) as f32,  position.1 as f32, 60.0, 60.0);
            
            
            d.draw_texture_pro(
                p_text,
                Rectangle {
                    x:0.0,
                    y:0.0,
                    width: p_text.width() as f32,
                    height: p_text.height() as f32,
                },
                left,
                Vector2 { x: 0.0, y: 0.0 },
                0.0,
                Color::WHITE,
            );
        }
        
    }
    
    fn piece_index(str: &String) -> (Result<usize, Error>, u8) {
        if str.contains("white") { 
            (Ok(str.find("/white").unwrap()), 1)
        } 
        else { (Ok(str.find("/black").unwrap()), 2) }
    } 

    pub fn load_pieces_textures(&mut self, rl: &mut RaylibHandle, thread: RaylibThread) -> Result::<(), Error> {
        for file in 
            glob("./imgs/pieces-basic-png/*")
            .expect("Directory not found") {
            
            match file {
                Ok(path) => {
                    let texture = rl.load_texture(&thread, 
                                                       path
                                                                .clone()
                                                                .into_os_string()
                                                                .to_str()
                                                                .expect("failed"));
                    
                    let path_substring = &(path.clone().into_os_string().into_string().unwrap());;

                    let (index, owner) = Self::piece_index(&String::from(path_substring));
                    
                    let name = string_to_piece_name(&path_substring[index? + 7..path_substring.len() - 4], texture.unwrap());

                    if !self.piece_textures.contains_key(&owner) {
                        let mut texture_vec = Vec::<PieceName>::new();
                        texture_vec.push(name.unwrap());
                        self.piece_textures.insert(owner, texture_vec);
                    } else {
                        self.piece_textures.insert(owner, v)
                    }
                },

                _ => panic!("bitch")
            }
        }
        Ok(())
    }

    pub fn run(&mut self, rl: &mut RaylibHandle, thread: RaylibThread) -> Result<(), Error> {
        // Load all the textures for each piece first
        self.load_pieces_textures(rl, thread.clone())?;

        let mut dragging = false;
        let mut offset = Vector2::default();
        
        while !(rl.window_should_close()) {
            // Begin drawing the textures after loading
            let mut d: &mut RaylibDrawHandle<'_> = &mut rl.begin_drawing(&thread);
            // clear background each frame
            d.clear_background(Color::WHITE);
            
            // Draw the board and it's alternating color spaces
            // pass over mutable reference to draw handle

            self.draw_board(&mut d);

            for s in 0..self.layout.len() {
                
            }
        };
        Ok(())
        
    }

    pub fn default() -> Game<'a> {
        
        let mut layout: [[u8; 8]; 8] = [[0; 8]; 8];
        let mut piece_order = vec![2, 3, 4, 5, 6, 4, 3, 2];

        let mut init_rows: Vec<u8> = (0..8).collect();
        // Player 1 pieces (leftmost = 1)
        // Player 2 Pieces (leftmost = 2)
        // If left most is 0, there is not piece there
        while !init_rows.is_empty() {
            let row = init_rows.pop();

            for col in 0..layout[row.unwrap() as usize].len() {
                layout[row.unwrap() as usize][col] += col as u8;
                
                let owner: u8 = match row {
                    Some(0) | Some(1) => { 1 },
                    Some(6) | Some(7) => { 2 },
                    _ => { 0 }
                };
                
                layout[row.unwrap() as usize][col] += owner * 64;

                match row {
                    Some(0) | Some(7) => {
                        layout[row.unwrap() as usize][col] += piece_order[col] * 8;
                    },        
                    Some(1) | Some(6) => {
                        layout[row.unwrap() as usize][col] += 8;
                    },
                    _ => {
                        continue;
                    },
                };
                
            }
        }
        
        Game {
            board: Board::<'a>::new(),
            turn: true,
            layout: layout,
            piece_textures: HashMap::<u8, Vec<PieceName>>::new(),
    // Map the names of pieces to the number of them left
    // per player
        }
    }
}