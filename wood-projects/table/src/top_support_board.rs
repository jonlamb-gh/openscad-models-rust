use crate::config::*;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use nalgebra::Vector3;
use parts::common_functions::z_axis;
use parts::Board;
use scad::*;

qstruct!(TopSupportBoard(color: Option<&'static str>) {
    board: Board = Board::from_array(&TOP_SUPPORT_BOARD_SIZE, color),
});

// TODO - enum for left/right like mortise side board?

impl TopSupportBoard {
    // TODO - trait
    // move to Part/part.rs?
    // TODO - center?
    pub fn abs_pos(left_side: bool) -> Vector3<f32> {
        let center = Vector3::new(TOTAL_SIZE[0] / 2.0, 0.0, 0.0);

        let mw = TOP_SUPPORT_BOARD_WIDTH;
        let hlegt = LEG_THICKNESS / 2.0;

        let hmajor = MAJOR_LEG_TO_LEG_DIST / 2.0;

        let t = if left_side {
            Vector3::new(-hmajor + hlegt, TOP_SUPPORT_BOARD_INSET, 0.0)
        } else {
            Vector3::new(hmajor - hlegt - mw, TOP_SUPPORT_BOARD_INSET, 0.0)
        };

        center + t
    }

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
