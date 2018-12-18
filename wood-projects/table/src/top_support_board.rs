use crate::config::*;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::z_axis;
use parts::Board;
use scad::*;

qstruct!(TopSupportBoard(color: Option<&'static str>) {
    board: Board = Board::from_array(&TOP_SUPPORT_BOARD_SIZE, color),
});

impl TopSupportBoard {
    pub fn assemble_aligned(&self) -> ScadObject {
        scad!(Translate(vec3(0.0, self.board.describe().length, 0.0));{
            scad!(Rotate(-90.0, z_axis());{
                self.board.assemble()
            })
        })
    }
}

impl ObjectAssembler for TopSupportBoard {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        self.assemble_aligned()
    }
}
