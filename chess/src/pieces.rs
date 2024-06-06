#[derive(Debug, Clone)]
pub enum Color {
    White,
    Black,
}
#[derive(Debug, Clone)]
pub struct Piece {
    pub symbol: String,
    pub notation: Option<char>,
    pub value: i32,
    pub color: Color,
}
impl Piece {
    fn new(name: &str, notation: char, value: i32, color: Color) -> Piece {
        Piece {
            symbol: name.to_string(),
            notation: Some(notation),
            value,
            color,
        }
    }
}

pub fn get_pieces() -> Vec<Vec<Piece>> {
    let mut pieces = Vec::new();
    let piece_types = vec![
        (("♙", "♟"), 'P', 1, 8),
        (("♖", "♜"), 'R', 5, 1),
        (("♘", "♞"), 'N', 3, 1),
        (("♗", "♝"), 'B', 3, 1),
        (("♕", "♛"), 'K', 9, 1),
        (("♔", "♚"), 'Q', 0, 1),
        (("♗", "♝"), 'B', 3, 1),
        (("♘", "♞"), 'N', 3, 1),
        (("♖", "♜"), 'R', 5, 1),
    ];

    for (name, notation, value, count) in &piece_types {
        for _ in 0..*count {
            pieces.push(Piece::new(
                name.1,
                notation.to_owned(),
                *value,
                Color::White,
            ));
        }
    }

    for (name, notation, value, count) in &piece_types {
        for _ in 0..*count {
            pieces.push(Piece::new(
                name.0,
                notation.to_owned(),
                *value,
                Color::Black,
            ));
        }
    }
    let nested_pieces: Vec<Vec<Piece>> = pieces.chunks(8).map(|chunk| chunk.to_vec()).collect();

    nested_pieces
}
