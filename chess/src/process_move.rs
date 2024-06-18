use crate::move_pieces::Current_move_string;
use crate::pieces::Piece;
pub fn process(to_move: Current_move_string, board: Vec<Vec<Piece>>) -> Vec<Vec<Piece>> {
    return board;
}

pub fn letter_to_number(letter: char) -> usize {
    match letter {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 404,
    }
}
