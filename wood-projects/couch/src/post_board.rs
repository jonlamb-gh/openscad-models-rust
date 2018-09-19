// TODO - this level of abstraction seems unnecessary
use scad::*;

use board::Board;
use config::POST_BOARD_SIZE;
use object_assembler::ObjectAssembler;

qstruct!(PostBoard() {
    board: Board = Board::from_array(&POST_BOARD_SIZE),
});

impl ObjectAssembler for PostBoard {
    fn assemble(&self) -> ScadObject {
        self.board.assemble()
    }
}
