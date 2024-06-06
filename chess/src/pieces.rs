#[derive(Debug, Clone)]
pub enum Color {
    White,
    Black,
}
#[derive(Debug, Clone)]
pub struct Piece {
    pub name: String,
    pub value: i32,
    pub color: Color,
}
impl Piece {
    fn new(name: &str, value: i32, color: Color) -> Piece {
        Piece {
            name: name.to_string(),
            value,
            color,
        }
    }
}

pub fn get_pieces() -> Vec<Vec<Piece>> {
    let mut pieces = Vec::new();
    let piece_types = vec![
        (("♙", "♟"), 1, 8),
        (("♖", "♜"), 5, 1),
        (("♘", "♞"), 3, 1),
        (("♗", "♝"), 3, 1),
        (("♕", "♛"), 9, 1),
        (("♔", "♚"), 0, 1),
        (("♗", "♝"), 3, 1),
        (("♘", "♞"), 3, 1),
        (("♖", "♜"), 5, 1),
    ];

    for (name, value, count) in &piece_types {
        for _ in 0..*count {
            pieces.push(Piece::new(name.1, *value, Color::White));
        }
    }

    for (name, value, count) in &piece_types {
        for _ in 0..*count {
            pieces.push(Piece::new(name.0, *value, Color::Black));
        }
    }
    let nested_pieces: Vec<Vec<Piece>> = pieces.chunks(8).map(|chunk| chunk.to_vec()).collect();

    nested_pieces
}
