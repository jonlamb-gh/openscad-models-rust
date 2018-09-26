use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::Board;
use scad::*;

use foundation::Foundation;

qstruct!(House() {
    board: Board = Board::new(50.0, 40.0, 50.0, None),
    foundation: Foundation = Foundation::new(vec3(1000.0, 1000.0, 5.0), Some("Gainsboro")),
});

impl ObjectAssembler for House {
    fn describe(&self) -> ObjectDescriptor {
        self.foundation.describe()
    }

    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.board.assemble(),
            self.foundation.assemble(),
        })
    }
}
