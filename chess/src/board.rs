pub use crate::pieces::{Color, Piece};

pub const WHITE_SQUARE: Piece = Piece {
    symbol: "■",
    notation: None,
    value: 0,
    color: Color::White,
};
pub const BLACK_SQUARE: Piece = Piece {
    symbol: "□",
    notation: None,
    value: 0,
    color: Color::Black,
};

pub fn get_board() -> Vec<Vec<Piece>> {
    let mut board = Vec::new();
    for _ in 0..4 {
        let mut file_odd = Vec::new();
        for i in 0..8 {
            if i % 2 == 0 {
                file_odd.push(WHITE_SQUARE);
            } else {
                file_odd.push(BLACK_SQUARE);
            }
        }
        board.push(file_odd);

        let mut file_even = Vec::new();
        for i in 0..8 {
            if i % 2 == 0 {
                file_even.push(BLACK_SQUARE);
            } else {
                file_even.push(WHITE_SQUARE);
            }
        }
        board.push(file_even);
    }
    return board;
}
