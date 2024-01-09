use raylib::prelude::*;
use raylib::consts::MouseButton::*;


pub enum PieceName {
    Pawn(Texture2D),
    Rook(Texture2D),
    Knight(Texture2D),
    Bishop(Texture2D),
    Queen(Texture2D),
    King(Texture2D)
}


trait Behvaior {
    fn can_move(&self);
}

impl Behvaior for PieceName {
    fn can_move(&self) {
    
    }
}


pub struct Piece<'a> {
    _phantom_data: std::marker::PhantomData<&'a ()>,
    pub owner: bool,
    pub name: PieceName,
    pub piece_texture: Texture2D,
    pub position: (u8, u8),
}


trait Movement {
    fn can_move(self);
}

impl Movement for PieceName {
    fn can_move(self) {
        
    }
}

impl<'a> Piece<'_> {
    pub fn draw_self(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.piece_texture, self.position.0.into(), self.position.1.into(), color::Color::WHITE);
    }

    pub fn _move(self, piece_rect: &mut Rectangle, d: &mut RaylibDrawHandle, rl: &mut RaylibHandle) {
        let mut dragging = false;
        let mut offset = Vector2::default();
        if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
            if piece_rect.check_collision_point_rec(rl.get_mouse_position()) {
                dragging = true;
                offset.x = rl.get_mouse_x() as f32 - piece_rect.x as f32; 
                offset.y = rl.get_mouse_y() as f32 - piece_rect.y as f32;
            }
        } 
        else if rl.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
            dragging = false;
        }
    }

    pub fn new(owner: bool, name: PieceName, piece_texture: Texture2D, position: (u8, u8), _phantom_marker: std::marker::PhantomData<&'a ()>) -> Piece<'a> {
        Piece {
            owner: owner,
            name: name,
            piece_texture: piece_texture,
            position: position, 
            _phantom_data: _phantom_marker,
        }
    }
}