use crate::config::*;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::Board;
use scad::*;

qstruct!(TopBoard(color: Option<&'static str>) {
    board: Board = Board::from_array(&TOP_BOARD_SIZE, color),
});

impl TopBoard {
    pub fn assemble_aligned(&self) -> ScadObject {
        self.board.assemble()
    }
}

impl ObjectAssembler for TopBoard {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        self.assemble_aligned()
    }
}
