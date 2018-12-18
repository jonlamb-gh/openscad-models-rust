use crate::config::*;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Board;
use scad::*;

qstruct!(Leg(color: Option<&'static str>) {
    board: Board = Board::from_array(&LEG_BOARD_SIZE, color),
});

impl Leg {
    pub fn assemble_aligned(&self) -> ScadObject {
        scad!(Translate(vec3(self.board.thickness(), 0.0, 0.0));{
            scad!(Rotate(-90.0, y_axis());{
                self.board.assemble()
            })
        })
    }
}

impl ObjectAssembler for Leg {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        // TODO
        self.assemble_aligned()
    }
}
