use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::Board;
use scad::*;

use foundation::Foundation;

qstruct!(House() {
    board: Board = Board::new(3.0, 4.0, 5.0, None),
    foundation: Foundation = Foundation::new(vec3(10.0, 10.0, 1.0)),
});

impl ObjectAssembler for House {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.board.assemble(),
            self.foundation.assemble(),
        })
    }
}
