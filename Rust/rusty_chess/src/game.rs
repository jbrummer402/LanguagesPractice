use crate::board::piece::{Piece, PieceType};
use glob::glob;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use std::collections::HashMap;
use std::fmt::{write, Error};
use std::{panic, u8};

fn string_to_piece_name(s: &str) -> Result<PieceType, Error> {
    match s {
        "pawn" | "Pawn" => Ok(PieceType::Pawn),
        "rook" | "Rook" => Ok(PieceType::Rook),
        "bishop" | "Bishop" => Ok(PieceType::Bishop),
        "knight" | "Knight" => Ok(PieceType::Knight),
        "king" | "King" => Ok(PieceType::King),
        "queen" | "Queen" => Ok(PieceType::Queen),
        _ => panic!("Piece not found! Zoinks!"),
    }
}

pub struct Game {
    pub turn: bool,
    pub layout: [[u8; 8]; 8],
    // Map the names of pieces to the number of them left
    piece_textures: HashMap<PieceType, Vec<Texture2D>>,
    pieces_left: Vec<Vec<Piece>>,
}


impl Game {
    fn draw_board(&mut self, d: &mut RaylibDrawHandle) {
        for row in 0..8 {
            for col in 0..8 {
                d.draw_rectangle(
                    60 * row,
                    60 * col,
                    60,
                    60,
                    if (col + row) % 2 == 0 {
                        color::Color::LIGHTGRAY
                    } else {
                        color::Color::GRAY
                    },
                );

                let space = self.layout[row as usize][col as usize];

                let owner = space / 64;
                let piece = (space / 8) % 8;
                let column = space % 8;

                if owner == 0 {
                    continue;
                }
            }
        }
    }
    fn create_pieces(&mut self) {
        for row in 0..self.layout.len() { 
            let mut piece_rows = Vec::<Piece>::new();
            
            for col in 0..self.layout[row].len() {
                

                let owner = self.layout[row as usize][col as usize] / 64;
                let piece = (self.layout[row as usize][col as usize] / 8) % 8;
                let column = self.layout[row as usize][col as usize] % 8;

                if owner == 0 {
                    continue;
                }

                let mut rect = 
                    Rectangle {
                        x: col as f32 * 60.0,
                        y: row as f32 * 60.0,
                        width: 60.0,
                        height: 60.0,
                    };
                
                let p_type = Piece::piece_to_name(piece);

                let mut new_piece = Piece {
                    rect: rect,
                    r#type: p_type,
                    is_dragging: false,
                };

                piece_rows.push(new_piece);
            }
            &self.pieces_left.push(piece_rows);
        }
    }
    fn draw_pieces(&self, d: &mut RaylibDrawHandle) {
        // Iterate through every piece currently on the board

        for row in 0..self.layout.len() {
            for col in 0..self.layout[row].len() {

                let space = self.layout[row][col];

                // Map the owner to the respective color
                // The piece to the respective texture
                // The column to the respective space
                let owner = space / 64;
                let piece = (space / 8) % 8;
                let column = space % 8;

                if owner == 0 {
                    continue;
                }

                let p_text: &_ = &self
                    .piece_textures
                    .get(&(Piece::piece_to_name(piece)))
                    .unwrap()[owner as usize - 1];

                d.draw_texture_pro(
                    p_text,
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: p_text.width() as f32,
                        height: p_text.height() as f32,
                    },
                    self.pieces_left[row as usize][col as usize].rect,
                    Vector2 { x: 0.0, y: 0.0 },
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }

    fn piece_index(str: &String) -> (Result<usize, Error>, u8) {
        if str.contains("white") {
            (Ok(str.find("white").unwrap()), 1)
        } else {
            (Ok(str.find("black").unwrap()), 2)
        }
    }

    fn load_pieces_textures(
        &mut self,
        rl: &mut RaylibHandle,
        thread: RaylibThread,
    ) -> Result<(), Error> {

        for file in glob("./imgs/pieces-basic-png/*").expect("Directory not found") {
            let f = file.unwrap();
            let texture = rl.load_texture(
                &thread,
                f.clone().into_os_string().to_str().expect("failed"),
            );
            let t = texture.unwrap();

            let path_substring = &(f.clone().into_os_string().into_string().unwrap());

            let (index, owner) = Self::piece_index(&String::from(path_substring));

            let name = string_to_piece_name(&path_substring[index? + 6..path_substring.len() - 4]);

            let p_name = &name.unwrap();

            match self.piece_textures.entry(*p_name) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push(t);
                }

                std::collections::hash_map::Entry::Vacant(mut entry) => {
                    let mut texture_vec = Vec::<Texture2D>::new();
                    texture_vec.push(t);
                    self.piece_textures.insert(*p_name, texture_vec);
                }
            }
        }
        self.create_pieces();
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

            for i in 0..self.pieces_left.len() {
                for j in 0..self.pieces_left[i].len() {
                    if d.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
                        if self.pieces_left[i][j].rect.check_collision_point_rec(d.get_mouse_position()) {
                            self.pieces_left[i][j].is_dragging = true;
                            offset.x = d.get_mouse_x() as f32 - self.pieces_left[i][j].rect.x as f32;
                            offset.y = d.get_mouse_y() as f32 - self.pieces_left[i][j].rect.y as f32;

                        }
                    } else if d.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
                        self.pieces_left[i][j].is_dragging = false;
                    }
                    if self.pieces_left[i][j].is_dragging {
                        self.pieces_left[i][j].rect.x = d.get_mouse_x() as f32 - offset.x;
                        self.pieces_left[i][j].rect.y = d.get_mouse_y() as f32 - offset.y;
                    }

                }

            }   
            self.draw_board(&mut d);
            self.draw_pieces(d);                
            
            let mut offset = Vector2::default();
        }
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
                    Some(0) | Some(1) => 1,
                    // elif the rows are the bottom two, the owner is player 2
                    Some(6) | Some(7) => 2,
                    _ => 0,
                };

                layout[row.unwrap() as usize][col] += owner * 64;

                match row {
                    Some(0) | Some(7) => {
                        layout[row.unwrap() as usize][col] += piece_order[col] * 8;
                    }
                    Some(1) | Some(6) => {
                        layout[row.unwrap() as usize][col] += 8;
                    }
                    _ => {
                        continue;
                    }
                };

            }
        }
        Game {
            turn: true,
            layout: layout,
            piece_textures: HashMap::<PieceType, Vec<Texture2D>>::new(),
            pieces_left: Vec::<Vec::<Piece>>::new(),

            // Map the names of pieces to the number of them left
            // per player
        }
    }
}
