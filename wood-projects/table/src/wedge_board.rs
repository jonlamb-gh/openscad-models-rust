use crate::config::*;
use crate::quadrant::Quadrant;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use nalgebra::Vector3;
use parts::common_functions::{y_axis, z_axis};
use parts::Board;
use scad::*;

qstruct!(WedgeBoard(color: Option<&'static str>) {
    board: Board = Board::from_array(&WEDGE_BOARD_SIZE, color),
});

impl WedgeBoard {
    // TODO - trait
    // move to Part/part.rs?
    // TODO - center?
    pub fn abs_pos(quad: Quadrant) -> Vector3<f32> {
        let center = Vector3::new(TOTAL_SIZE[0] / 2.0, TOTAL_SIZE[1] / 2.0, 0.0);

        let z = SIDE_SUPPORT_BOARD_HEIGHT - (SIDE_SUPPORT_BOARD_WIDTH / 2.0) - (WEDGE_WIDTH / 2.0);
        let hmajor = TENON_SIDE_SUPPORT_BOARD_LENGTH / 2.0;
        let hminor = MORTISE_SIDE_SUPPORT_BOARD_LENGTH / 2.0;
        let tor = TENON_OVERRUN;
        let hl = WEDGE_LENGTH / 2.0;
        let wt = WEDGE_THICKNESS;
        let dy = MORTISE_BOARD_TENON_OVERRUN;
        let dy1 = MORTISE_BOARD_TENON_OVERRUN + (LEG_WIDTH / 2.0);

        let t = match quad {
            Quadrant::Q0 => Vector3::new(-hmajor + tor - wt, -hminor - dy + hl, z),
            Quadrant::Q1 => Vector3::new(-hmajor + tor - wt, hminor - dy1 - hl, z),
            Quadrant::Q2 => Vector3::new(hmajor - tor, hminor - dy1 - hl, z),
            Quadrant::Q3 => Vector3::new(hmajor - tor, -hminor - dy + hl, z),
        };

        center + t
    }

    pub fn assemble_aligned(&self) -> ScadObject {
        self.assemble()
    }
}

impl ObjectAssembler for WedgeBoard {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        scad!(Rotate(90.0, y_axis());{
            scad!(Rotate(90.0, z_axis());{
                self.board.assemble()
            })
        })
    }
}
