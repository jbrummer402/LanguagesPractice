use raylib::core::texture::Image;

enum PieceName {
    pawn,
    bishop,
    knight,
    rook,
    king, 
    queen, 
}

pub struct Piece {
    owner: bool,
    name: PieceName,
    position: (u8, u8),
    img: Image,
}

impl Piece {
    pub fn new(owner: bool, name: PieceName, position: (u8, u8), img: Image) -> Piece {
        Self {
            owner: owner,
            name: name,
            position: position,
            img: img,
        }
    }
}