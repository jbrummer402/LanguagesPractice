use raylib::{prelude::*, ffi::posix_openpt};
use crate::board::Board;
use crate::board::piece::{Piece, PieceName};
use crate::board::piece;
use std::collections::HashMap;

use std::panic;
use std::{path::{PathBuf}, num::ParseIntError, fmt::Error, sync::TryLockResult};
use glob::glob;

use substring::Substring;


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

#[derive(Default)]
pub struct Game {
    player_1_score: u8,
    white_textures: HashMap<PieceName, Texture2D>,
    black_textures: HashMap<PieceName, Texture2D>,
}

impl Game {
    pub fn draw_pieces(d: &mut RaylibDrawHandle, p_text: &Texture2D, position: (u16, u16), space_between: u16, num_pieces: u8) {
        let left: Rectangle = Rectangle::new(position.0 as f32,  position.1 as f32, 60.0, 60.0);
        d.draw_texture_pro(
            p_text,
            Rectangle {
                x:0.0,
                y:0.0,
                width: p_text.width() as f32,
                height: p_text.height() as f32,
            },
            left,
            Vector2 { x: 0.0, y: 0.0},
            0.0,
            Color::WHITE,
        );
        
        if num_pieces > 1 {
            let right: Rectangle = Rectangle::new((position.0 + 60 * space_between) as f32,  position.1 as f32, 60.0, 60.0);
        
            d.draw_texture_pro(
                p_text,
                Rectangle {
                    x:0.0,
                    y:0.0,
                    width: p_text.width() as f32,
                    height: p_text.height() as f32,
                },
                right,
                Vector2 { x: 0.0, y: 0.0},
                0.0,
                Color::WHITE,
            );
        }
        
    }
    fn piece_index(str: &String) -> (Result<usize, Error>, bool) {
        if str.contains("white") { 
            (Ok(str.find("/white").unwrap()), false)
        } 
        else { (Ok(str.find("/black").unwrap()), true ) }
    } 
    pub fn load_pieces_textures(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) -> Result::<(), Error> {
        for file in 
        glob("./imgs/pieces-basic-png/*")
        .expect("Directory not found") {
            match file {
                Ok(path) => {
                    let texture = rl.load_texture(thread, 
                                                       path.clone().into_os_string()
                                                                .to_str()
                                                                .expect("failed"));

                    let path_substring = &(path.clone().into_os_string().into_string().unwrap());

                    let (index, owner) = Self::piece_index(&String::from(path_substring));
                    println!("{:?}", &path_substring[index? + 5..path_substring.len()]);
                    
                    let name = string_to_piece_name(&path_substring[index? + 7..path_substring.len() - 4]);
                    if owner {
                        self.black_textures.insert(name?, texture.unwrap());
                    } else {
                        self.white_textures.insert(name?, texture.unwrap());
                    }
                }
                
                _ => panic!("panic")
            }
        }
        Ok(())
    }

    pub fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Initialize game board and load the pieces' textures
        let mut brd: Board = Board::new();
        
        self.load_pieces_textures(rl, thread);
        // let handle = raylib::prelude::init();
        while !(rl.window_should_close()) {
            let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
            brd.draw(&mut d);

            for b in &self.black_textures {
                match &b.0 {
                    PieceName::Rook => {
                        Self::draw_pieces(&mut d, &b.1, (0, 0), 7, 2);
                    },
                    PieceName::Knight => {
                        Self::draw_pieces(&mut d, &b.1, (60, 0), 5, 2);
                    },
                    PieceName::Bishop => {
                        Self::draw_pieces(&mut d, &b.1, (120, 0), 3, 2);
                    },
                    PieceName::King => {
                        Self::draw_pieces(&mut d, &b.1, (240, 0), 0, 1);
                    },
                    PieceName::Queen => {
                        Self::draw_pieces(&mut d, &b.1, (180, 0), 0, 1);
                    },
                    PieceName::Pawn => {
                        for col in &brd.spaces[0] {
                            let dest: Rectangle = Rectangle::new(col.position.1 as f32 * 60.0,  60.0, 60.0, 60.0);
                            
                            d.draw_texture_pro(
                                b.1,
                                Rectangle {
                                    x:0.0,
                                    y:0.0,
                                    width: b.1.width() as f32,
                                    height: b.1.height() as f32,
                                },
                                dest,
                                Vector2 { x: 0.0, y: 0.0},
                                0.0,
                                Color::WHITE,
                            );
                        }
                    },
                }
                
            }

            for w in &self.white_textures {
                match &w.0 {
                    PieceName::Rook => {
                        Self::draw_pieces(&mut d, &w.1, (0, 420), 7, 2);
                    },
                    PieceName::Knight => {
                        Self::draw_pieces(&mut d, &w.1, (60, 420), 5, 2);
                    },
                    PieceName::Bishop => {
                        Self::draw_pieces(&mut d, &w.1, (120, 420), 3, 2);
                    },
                    PieceName::King => {
                        Self::draw_pieces(&mut d, &w.1, (240, 420), 0, 1);
                    },
                    PieceName::Queen => {
                        Self::draw_pieces(&mut d, &w.1, (180, 420), 0, 1);
                    },
                    PieceName::Pawn => {
                        for col in &brd.spaces[0] {
                            let dest: Rectangle = Rectangle::new(col.position.1 as f32 * 60.0,  360.0, 60.0, 60.0);
                            
                            d.draw_texture_pro(
                                w.1,
                                Rectangle {
                                    x:0.0,
                                    y:0.0,
                                    width: w.1.width() as f32,
                                    height: w.1.height() as f32,
                                },
                                dest,
                                Vector2 { x: 0.0, y: 0.0},
                                0.0,
                                Color::WHITE,
                            );
                        }
                    },
                }
            }
        };
        
    }

    pub fn default() -> Game {
        Game {
            player_1_score: 0,
            white_textures: HashMap::<PieceName, Texture2D>::new(),
            black_textures: HashMap::<PieceName, Texture2D>::new(),
        }
    }
}