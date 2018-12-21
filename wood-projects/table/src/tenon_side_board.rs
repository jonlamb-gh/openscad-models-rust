use crate::axis::Axis;
use crate::config::*;
use crate::cutaway::Cutaway;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use nalgebra::Vector3;
use parts::common_functions::{x_axis, y_axis, z_axis};
use parts::Board;
use scad::*;

qstruct!(TenonSideBoard(color: Option<&'static str>) {
    board: Board = Board::from_array(&TENON_SIDE_SUPPORT_BOARD_SIZE, color),
});

pub enum TenonSide {
    /// Quandrants Q0 and Q3
    Front,
    /// Quandrants Q1 and Q2
    Back,
}

impl TenonSideBoard {
    // TODO - trait
    // move to Part/part.rs?
    // TODO - center?
    pub fn abs_pos(side: TenonSide) -> Vector3<f32> {
        let center = Vector3::new(TOTAL_SIZE[0] / 2.0, TOTAL_SIZE[1] / 2.0, 0.0);

        let z = SIDE_SUPPORT_BOARD_HEIGHT - SIDE_SUPPORT_BOARD_WIDTH;
        let hl = TENON_SIDE_SUPPORT_BOARD_LENGTH / 2.0;
        let ht = SIDE_SUPPORT_BOARD_THICKNESS / 2.0;
        let hminor = MINOR_LEG_TO_LEG_DIST / 2.0;

        let t = match side {
            TenonSide::Front => Vector3::new(-hl, -hminor - ht, z),
            TenonSide::Back => Vector3::new(-hl, hminor - ht, z),
        };

        center + t
    }

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

    fn left_wedge_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            TENON_OVERRUN - WEDGE_THICKNESS,
            (SIDE_SUPPORT_BOARD_WIDTH / 2.0) - (WEDGE_WIDTH / 2.0),
            -VISUAL_OVERRUN,
            // size
            WEDGE_THICKNESS,
            WEDGE_WIDTH,
            SIDE_SUPPORT_BOARD_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn right_wedge_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            TENON_SIDE_SUPPORT_BOARD_LENGTH - TENON_OVERRUN,
            (SIDE_SUPPORT_BOARD_WIDTH / 2.0) - (WEDGE_WIDTH / 2.0),
            -VISUAL_OVERRUN,
            // size
            WEDGE_THICKNESS,
            WEDGE_WIDTH,
            SIDE_SUPPORT_BOARD_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn upper_tenon_cutaway(&self) -> Cutaway {
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

    fn lower_tenon_cutaway(&self) -> Cutaway {
        let offset = (LEG_THICKNESS / 2.0) - (SIDE_SUPPORT_BOARD_THICKNESS / 2.0);
        Cutaway::from_parts(
            // position
            -VISUAL_OVERRUN,
            -VISUAL_OVERRUN,
            -VISUAL_OVERRUN,
            // size
            SIDE_SUPPORT_TENON_LENGTH + VISUAL_OVERRUN - offset,
            (self.describe().width - SIDE_SUPPORT_TENON_WIDTH) / 2.0 + VISUAL_OVERRUN,
            self.describe().thickness + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn left_tenon_cutaways(&self) -> ScadObject {
        let y =
            (self.board.describe().width / 2.0) + (SIDE_SUPPORT_TENON_WIDTH / 2.0) + VISUAL_OVERRUN;
        scad!(Union;{
            self.lower_tenon_cutaway().assemble(),
            scad!(Translate(vec3(0.0, y, 0.0));{
                self.upper_tenon_cutaway().assemble(),
            })
        })
    }

    fn right_tenon_cutaways(&self) -> ScadObject {
        let x = self.board.describe().length - SIDE_SUPPORT_TENON_LENGTH + VISUAL_OVERRUN;
        let y =
            (self.board.describe().width / 2.0) + (SIDE_SUPPORT_TENON_WIDTH / 2.0) + VISUAL_OVERRUN;
        let offset = (LEG_THICKNESS / 2.0) - (SIDE_SUPPORT_BOARD_THICKNESS / 2.0);
        scad!(Translate(vec3(x + offset, 0.0, 0.0));{
            scad!(Union;{
                self.lower_tenon_cutaway().assemble(),
                scad!(Translate(vec3(-offset, y, 0.0));{
                    self.upper_tenon_cutaway().assemble(),
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
            self.left_wedge_cutaway().assemble(),
            self.right_wedge_cutaway().assemble(),
        })
    }
}
