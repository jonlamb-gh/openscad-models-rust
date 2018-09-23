// TODO - this level of abstraction seems unnecessary
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use board::Board;
use config::POST_BOARD_SIZE;

qstruct!(PostBoard() {
    board: Board = Board::from_array(&POST_BOARD_SIZE, Some("SandyBrown")),
});

impl ObjectAssembler for PostBoard {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        self.board.assemble()
    }
}
