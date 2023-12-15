use raylib::core::texture::Image;

enum PieceName {
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
    position: (u8, u8),
    img: Image,
}

impl Piece {
    fn new(owner: bool, name: PieceName, position: (u8, u8), img: Image) -> Piece {
        Self {
            owner: owner,
            name: name,
            position: position,
            img: img,
        }
    }
}