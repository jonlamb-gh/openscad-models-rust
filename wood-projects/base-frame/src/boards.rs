use crate::config::*;
use parts::prelude::*;

pub fn long_frame_board() -> Board {
    let dims = BoardDimensions::new(
        LONG_FRAME_BOARD_LENGTH,
        FRAME_BOARD_WIDTH,
        FRAME_BOARD_THICKNESS,
    );
    Board::with_color(dims, FRAME_BOARD_COLOR)
}

pub fn short_frame_board() -> Board {
    let dims = BoardDimensions::new(
        SHORT_FRAME_BOARD_LENGTH,
        FRAME_BOARD_WIDTH,
        FRAME_BOARD_THICKNESS,
    );
    Board::with_color(dims, FRAME_BOARD_COLOR)
}
