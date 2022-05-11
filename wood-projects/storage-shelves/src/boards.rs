use crate::config::*;
use parts::prelude::*;

pub fn long_board() -> Board<Inch> {
    let dims = BoardDimensions::new(LONG_BOARD_LENGTH, BOARD_WIDTH, BOARD_THICKNESS);
    Board::with_color(dims, LONG_BOARD_COLOR)
}

pub fn med_board() -> Board<Inch> {
    let dims = BoardDimensions::new(MED_BOARD_LENGTH, BOARD_WIDTH, BOARD_THICKNESS);
    Board::with_color(dims, MED_BOARD_COLOR)
}

pub fn short_board() -> Board<Inch> {
    let dims = BoardDimensions::new(SHORT_BOARD_LENGTH, BOARD_WIDTH, BOARD_THICKNESS);
    Board::with_color(dims, SHORT_BOARD_COLOR)
}
