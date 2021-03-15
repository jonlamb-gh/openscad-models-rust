use crate::config::*;
use parts::prelude::*;

pub fn long_frame_board() -> Board {
    let dims = BoardDimensions::new(
        LONG_FRAME_BOARD_LENGTH,
        FRAME_BOARD_WIDTH,
        FRAME_BOARD_THICKNESS,
    );
    Board::with_color(dims, LONG_FRAME_BOARD_COLOR)
}

pub fn short_frame_board() -> Board {
    let dims = BoardDimensions::new(
        SHORT_FRAME_BOARD_LENGTH,
        FRAME_BOARD_WIDTH,
        FRAME_BOARD_THICKNESS,
    );
    Board::with_color(dims, SHORT_FRAME_BOARD_COLOR)
}

pub fn slat_board() -> Board {
    let dims = BoardDimensions::new(SLAT_BOARD_LENGTH, SLAT_BOARD_WIDTH, SLAT_BOARD_THICKNESS);
    Board::with_color(dims, SLAT_BOARD_COLOR)
}
