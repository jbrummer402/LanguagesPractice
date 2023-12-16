use raylib::core::texture::Image;

pub enum PieceName {
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
    fn get_image(self, file_path: String) -> Image {
        Image::load_image(&file_path).unwrap()
    }

    pub fn new(self, owner: bool, name: PieceName, position: (u8, u8), file_path: String) -> Piece {
        Piece {
            owner: owner,
            name: name,
            position: position,
            img: self.get_image(String::from(file_path)),
        }
    }
}