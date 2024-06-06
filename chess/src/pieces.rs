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

pub fn get_pieces() -> Vec<Piece> {
    let mut pieces = Vec::new();
    let piece_types = vec![
        ("Pawn", 1, 8),
        ("Rook", 5, 2),
        ("Knight", 3, 2),
        ("Bishop", 3, 2),
        ("Queen", 9, 1),
        ("King", 0, 1),
    ];

    for (name, value, count) in &piece_types {
        for _ in 0..*count {
            pieces.push(Piece::new(name, *value, Color::White));
        }
    }

    for (name, value, count) in &piece_types {
        for _ in 0..*count {
            pieces.push(Piece::new(name, *value, Color::Black));
        }
    }

    pieces
}
