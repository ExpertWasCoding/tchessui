pub use crate::pieces::{Color, Piece};

pub fn get_board() -> Vec<Vec<Piece>> {
    let mut board = Vec::new();
    let white_square = Piece {
        symbol: String::from("■"),
        notation: None,
        value: 0,
        color: Color::White,
    };
    let black_square = Piece {
        symbol: String::from("□"),
        notation: None,
        value: 0,
        color: Color::Black,
    };

    for _ in 0..4 {
        let file_odd = vec![
            white_square.clone(),
            black_square.clone(),
            white_square.clone(),
            black_square.clone(),
            white_square.clone(),
            black_square.clone(),
            white_square.clone(),
            black_square.clone(),
        ];
        let file_even = vec![
            black_square.clone(),
            white_square.clone(),
            black_square.clone(),
            white_square.clone(),
            black_square.clone(),
            white_square.clone(),
            black_square.clone(),
            white_square.clone(),
        ];
        board.push(file_odd);
        board.push(file_even);
    }
    return board;
}
