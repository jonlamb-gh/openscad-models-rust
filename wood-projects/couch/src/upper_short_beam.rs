use scad::*;

use board::Board;
use config::*;
use cutaway::Cutaway;
use dimdraw::{ObjectAssembler, ObjectDescriptor};

qstruct!(UpperShortBeam() {
    board: Board = Board::from_array(&SHORT_BEAM_BOARD_SIZE),
});

impl UpperShortBeam {
    fn left_tenon_cutaway(&self) -> Cutaway {
        Cutaway::new(
            // position
            -VISUAL_OVERRUN,
            SHORT_BEAM_UPPER_FRONT_TENON_WIDTH,
            -VISUAL_OVERRUN,
            // size
            SHORT_BEAM_TENON_DEPTH + VISUAL_OVERRUN,
            BEAM_STOCK_WIDTH - SHORT_BEAM_UPPER_FRONT_TENON_WIDTH + VISUAL_OVERRUN,
            BEAM_STOCK_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn right_tenon_cutaway(&self) -> Cutaway {
        Cutaway::new(
            // position
            self.board.dims().length() - SHORT_BEAM_TENON_DEPTH,
            SHORT_BEAM_UPPER_REAR_TENON_WIDTH,
            -VISUAL_OVERRUN,
            // size
            SHORT_BEAM_TENON_DEPTH + VISUAL_OVERRUN,
            BEAM_STOCK_WIDTH - SHORT_BEAM_UPPER_REAR_TENON_WIDTH + VISUAL_OVERRUN,
            BEAM_STOCK_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )
    }
}

impl ObjectAssembler for UpperShortBeam {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        // align back to center and orientate
        let mut rot_sub = scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0)));
        let mut rot = scad!(Rotate(90.0, vec3(1.0, 0.0, 0.0)));

        // subtract the cutouts from the board
        let mut parent = scad!(Difference);
        parent.add_child(self.board.assemble());
        parent.add_child(self.left_tenon_cutaway().assemble());
        parent.add_child(self.right_tenon_cutaway().assemble());

        // combine and return a parent container object
        rot.add_child(parent);
        rot_sub.add_child(rot);
        rot_sub
    }
}
