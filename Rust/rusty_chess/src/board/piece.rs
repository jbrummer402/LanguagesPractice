use raylib::consts::MouseButton::*;
use raylib::prelude::*;

#[derive(Debug, Eq, Hash, Copy, Clone, PartialEq)]
pub enum PieceName {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

trait Behvaior {
    fn can_move(&self);
}

impl Behvaior for PieceName {
    fn can_move(&self) {}
}

pub struct Piece {
    pub owner: bool,
    pub name: PieceName,
    pub piece_rect: Rectangle,
    pub piece_texture: Texture2D,
}

trait Movement {
    fn can_move(self);
}

impl Movement for PieceName {
    fn can_move(self) {}
}

impl Piece {
    pub fn _move(
        self,
        piece_rect: &mut Rectangle,
        d: &mut RaylibDrawHandle,
        rl: &mut RaylibHandle,
    ) {
        let mut dragging = false;
        let mut offset = Vector2::default();
        if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
            if piece_rect.check_collision_point_rec(rl.get_mouse_position()) {
                dragging = true;
                offset.x = rl.get_mouse_x() as f32 - piece_rect.x as f32;
                offset.y = rl.get_mouse_y() as f32 - piece_rect.y as f32;
            }
        } else if rl.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
            dragging = false;
        }
    }
    pub fn piece_to_name(index: u8) -> PieceName {
        match index {
            1 => PieceName::Pawn,
            2 => PieceName::Rook,
            3 => PieceName::Knight,
            4 => PieceName::Bishop,
            5 => PieceName::Queen,
            6 => PieceName::King,
            _ => PieceName::Pawn,
        }
    }
    pub fn load_rects(rl: &mut RaylibHandle, layout: [[u8; 8]; 8]) -> Vec<Rectangle> {
        let mut piece_rects = Vec::with_capacity(32);

        for row in 0..layout.len() {
            for col in 0..layout[row].len() {
                let space = layout[row][col];

                let owner = space / 64;
                let piece = (space / 8) % 8;
                let location = space % 8;

                if owner == 0 {
                    continue;
                }

                piece_rects.push(Rectangle {
                    x: location as f32 * 60.0,
                    y: row as f32 * 60.0,
                    width: 60.0,
                    height: 60.0,
                });
            }
        }

        return piece_rects;
    }

    pub fn new(
        owner: bool,
        name: PieceName,
        piece_rect: Rectangle,
        piece_texture: Texture2D,
    ) -> Piece {
        Piece {
            owner: owner,
            name: name,
            piece_texture: piece_texture,
            piece_rect: piece_rect,
        }
    }
}
