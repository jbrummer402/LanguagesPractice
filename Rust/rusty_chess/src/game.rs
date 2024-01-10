use raylib::prelude::*;
use crate::board::{Board};
use crate::board::piece::{Piece, PieceName};
use std::collections::HashMap;

use std::{panic, u8};
use std::fmt::Error;
use glob::glob;

fn string_to_piece_name(s: &str) -> Result::<PieceName, Error> {
    match s {
        "pawn" => Ok(PieceName::Pawn),
        "rook" => Ok(PieceName::Rook),
        "bishop" => Ok(PieceName::Bishop),
        "knight" => Ok(PieceName::Knight),
        "king" => Ok(PieceName::King),
        "queen" => Ok(PieceName::Queen),
        _ => panic!("bitch"),
    }
}

pub struct Game {
    pub turn: bool,
    pub layout: [[u8; 8]; 8],
    // Map the names of pieces to the number of them left
    piece_textures: HashMap<PieceName, Vec<Texture2D>>,
}

impl Game {
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
    pub fn draw_pieces(&self, d: &mut RaylibDrawHandle) {
        
        // Iterate through every piece currently on the board
        for row in 0..self.layout.len() {
            for col in 0..self.layout[row].len() {
                // match col {
                //     8 => println!(),
                //     _ => println!("Not 8"),
                    
                // }
                
                let space = self.layout[row][col];

                let owner = space / 64;
                let piece = (space / 8) % 8;
                let location  = space % 8;                    
                
                // Map the owner to the respective color 
                // The piece to the respective texture
                // The location to the respective space
                let text_rect = Rectangle::new(
                    (col * 60) as f32,
                    (row * 60) as f32,
                    60.0,60.0
                );
                
                let p_text: &_ = match piece {
                    1 => {
                        &self.piece_textures.get(&PieceName::Pawn).unwrap()[owner as usize - 1]
                    },
                    2 => {
                        &self.piece_textures.get(&PieceName::Rook).unwrap()[owner as usize - 1]
                    },
                    3 => {
                        &self.piece_textures.get(&PieceName::Knight).unwrap()[owner as usize - 1]
                    },
                    4 => {
                        &self.piece_textures.get(&PieceName::Bishop).unwrap()[owner as usize - 1]
                    },
                    5 => {
                        &self.piece_textures.get(&PieceName::Queen).unwrap()[owner as usize - 1]
                    },
                    6 => {
                        &self.piece_textures.get(&PieceName::King).unwrap()[owner as usize - 1]
                    },
                    _ => continue,
                };
                
                d.draw_texture_pro(
                    p_text, 
                    Rectangle {
                     x:0.0,
                     y:0.0,
                     width: p_text.width() as f32,
                     height: p_text.height() as f32, },
                    text_rect,
                    Vector2 {x: 0.0, y: 0.0},
                    0.0,
                    Color::WHITE,
                );               
            }
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
                    let f = file.unwrap();    
                    let texture = rl.load_texture(&thread, 
                                                    f 
                                                    .clone()
                                                    .into_os_string()
                                                    .to_str()
                                                    .expect("failed"));
                    let t = texture.unwrap();

                    let path_substring = &(f.clone().into_os_string().into_string().unwrap());

                    let (index, owner) = Self::piece_index(&String::from(path_substring));
             
                    let name = string_to_piece_name(&path_substring[index? + 7..path_substring.len() - 4]);
                    let p_name = &name.unwrap();

                    match self.piece_textures.entry(*p_name) {
                        std::collections::hash_map::Entry::Occupied(mut entry) => {
                            entry.get_mut().push(t);
                        },
                        std::collections::hash_map::Entry::Vacant(mut entry) => {
                        let mut texture_vec = Vec::<Texture2D>::new();
                        texture_vec.push(t);
                        self.piece_textures.insert(*p_name, texture_vec);
                        },

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
            self.draw_pieces(d);

        };
        Ok(())
        
    }
    pub fn default() -> Game {
        
        let mut layout: [[u8; 8]; 8] = [[0; 8]; 8];
        let mut piece_order = vec![2, 3, 4, 5, 6, 4, 3, 2];

        let mut init_rows: Vec<u8> = (0..8).collect();
        // Player 1 pieces (leftmost = 1)
        // Player 2 Pieces (leftmost = 2)
        // If left most is 0, there is not piece there
        while !init_rows.is_empty() {
            let row = init_rows.pop();

            for col in 0..layout[row.unwrap() as usize].len() {
                // Position of the piece
                layout[row.unwrap() as usize][col] += col as u8;
                
                let owner: u8 = match row {
                    // If the rows are the top two, the owner is player 1
                    Some(0) | Some(1) => { 1 },
                    // elif the rows are the bottom two, the owner is player 2
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
            turn: true,
            layout: layout,
            piece_textures: HashMap::<PieceName, Vec<Texture2D>>::new(),
    // Map the names of pieces to the number of them left
    // per player
        }
    }
}
    
