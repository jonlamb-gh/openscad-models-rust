use crate::config::*;
use crate::cutaway::Cutaway;
use crate::side_board::Axis;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::{x_axis, y_axis, z_axis};
use parts::Board;
use scad::*;

qstruct!(TenonSideBoard(color: Option<&'static str>) {
    board: Board = Board::from_array(&TENON_SIDE_SUPPORT_BOARD_SIZE, color),
});

impl TenonSideBoard {
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

    fn tenon_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            -VISUAL_OVERRUN,
            -VISUAL_OVERRUN,
            -VISUAL_OVERRUN,
            // size
            SIDE_SUPPORT_TENON_LENGTH + VISUAL_OVERRUN,
            (self.describe().width - SIDE_SUPPORT_TENON_WIDTH) / 2.0 + VISUAL_OVERRUN,
            self.describe().thickness + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn left_tenon_cutaways(&self) -> ScadObject {
        let y =
            (self.board.describe().width / 2.0) + (SIDE_SUPPORT_TENON_WIDTH / 2.0) + VISUAL_OVERRUN;
        scad!(Union;{
            self.tenon_cutaway().assemble(),
            scad!(Translate(vec3(0.0, y, 0.0));{
                self.tenon_cutaway().assemble(),
            })
        })
    }

    fn right_tenon_cutaways(&self) -> ScadObject {
        let x = self.board.describe().length - SIDE_SUPPORT_TENON_LENGTH + VISUAL_OVERRUN;
        let y =
            (self.board.describe().width / 2.0) + (SIDE_SUPPORT_TENON_WIDTH / 2.0) + VISUAL_OVERRUN;
        scad!(Translate(vec3(x, 0.0, 0.0));{
            scad!(Union;{
                self.tenon_cutaway().assemble(),
                scad!(Translate(vec3(0.0, y, 0.0));{
                    self.tenon_cutaway().assemble(),
                })
            })
        })
    }
}

impl ObjectAssembler for TenonSideBoard {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        scad!(Difference;{
            self.board.assemble(),
            self.left_tenon_cutaways(),
            self.right_tenon_cutaways(),
        })
    }
}
