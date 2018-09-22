use scad::*;

use board::Board;
use config::*;
use dimdraw::{ObjectAssembler, ObjectDescriptor};

qstruct!(SupportPlank() {
    board: Board = Board::from_array(&SUPPORT_PLANK_BOARD_SIZE),
});

impl ObjectAssembler for SupportPlank {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        // align back to center and orientate
        let mut pos = scad!(Translate(vec3(self.board.dims().width(), 0.0, 0.0)));
        let mut rot = scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0)));

        // combine and return a parent container object
        rot.add_child(self.board.assemble());
        pos.add_child(rot);
        pos
    }
}
