use scad::*;

use board_dimensions::BoardDimensions;
use config::*;
use cutaway::Cutaway;
use object_assembler::ObjectAssembler;
use post_board::PostBoard;

pub enum Loc {
    LeftFront,
    LeftRear,
    RightFront,
    RightRear,
}

qstruct!(Post(loc: Loc) {
    board: PostBoard = PostBoard::new(),
    loc: Loc = loc,
});

impl Post {
    fn lower_short_beam_tenon_cutaway(&self) -> Cutaway {
        Cutaway::new(
            // position
            BASE_HEIGHT + BEAM_STOCK_WIDTH - SHORT_BEAM_LOWER_TENON_WIDTH,
            -VISUAL_OVERRUN,
            (POST_STOCK_THICKNESS - BEAM_STOCK_THICKNESS) / 2.0,
            // size
            SHORT_BEAM_LOWER_TENON_WIDTH,
            POST_STOCK_THICKNESS + (2.0 * VISUAL_OVERRUN),
            BEAM_STOCK_THICKNESS,
        )
    }
}

impl ObjectAssembler for Post {
    fn assemble(&self) -> ScadObject {
        let mut parent = scad!(Difference);

        parent.add_child(self.board.assemble());

        parent.add_child(self.lower_short_beam_tenon_cutaway().assemble());

        parent
    }
}
