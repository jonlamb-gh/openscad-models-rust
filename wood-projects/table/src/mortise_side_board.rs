use crate::config::*;
use crate::cutaway::Cutaway;
use crate::side_board::Axis;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::{x_axis, y_axis, z_axis};
use parts::Board;
use scad::*;

qstruct!(MortiseSideBoard(color: Option<&'static str>) {
    board: Board = Board::from_array(&MORTISE_SIDE_SUPPORT_BOARD_SIZE, color),
});

impl MortiseSideBoard {
    pub fn assemble_aligned(&self, axis: Axis) -> ScadObject {
        let dim = self.board.describe();
        match axis {
            Axis::X => scad!(Translate(vec3(0.0, dim.thickness, 0.0));{
                    scad!(Rotate(90.0, x_axis());{
                        self.assemble()
                    })
                }),
            Axis::Y => scad!(Translate(vec3(dim.thickness, dim.length, 0.0));{
                    scad!(Rotate(-90.0, y_axis());{
                        scad!(Rotate(-90.0, z_axis());{
                            self.assemble()
                        })
                    })
                }),
        }
    }

    fn left_tenon_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            MORTISE_BOARD_TENON_OVERRUN + (POST_WIDTH / 2.0)
                - (self.board.describe().thickness / 2.0),
            (self.describe().width / 2.0) - (SIDE_SUPPORT_MORTISE_WIDTH / 2.0),
            -VISUAL_OVERRUN,
            // size
            SIDE_SUPPORT_MORTISE_LENGTH,
            SIDE_SUPPORT_MORTISE_WIDTH,
            self.describe().thickness + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn right_tenon_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            self.describe().length
                - MORTISE_BOARD_TENON_OVERRUN
                - (POST_WIDTH / 2.0)
                - (self.board.describe().thickness / 2.0),
            (self.describe().width / 2.0) - (SIDE_SUPPORT_MORTISE_WIDTH / 2.0),
            -VISUAL_OVERRUN,
            // size
            SIDE_SUPPORT_MORTISE_LENGTH,
            SIDE_SUPPORT_MORTISE_WIDTH,
            self.describe().thickness + (2.0 * VISUAL_OVERRUN),
        )
    }
}

impl ObjectAssembler for MortiseSideBoard {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        scad!(Difference;{
            self.board.assemble(),
            self.left_tenon_cutaway().assemble(),
            self.right_tenon_cutaway().assemble(),
        })
    }
}
