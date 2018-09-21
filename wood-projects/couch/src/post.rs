use dimdraw::ObjectAssembler;
use scad::*;

use config::*;
use cutaway::Cutaway;
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
            POST_STOCK_WIDTH + (2.0 * VISUAL_OVERRUN),
            BEAM_STOCK_THICKNESS,
        )
    }

    fn lower_long_beam_tenon_cutaway(&self) -> Cutaway {
        Cutaway::new(
            // position
            BASE_HEIGHT - LONG_BEAM_TENON_WIDTH,
            (POST_STOCK_THICKNESS - BEAM_STOCK_THICKNESS) / 2.0,
            -VISUAL_OVERRUN,
            // size
            LONG_BEAM_TENON_WIDTH,
            BEAM_STOCK_THICKNESS,
            POST_STOCK_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn front_upper_short_beam_tenon_cutaway(&self) -> Cutaway {
        Cutaway::new(
            // position
            BASE_POST_HEIGHT - SIDE_ARM_THICKNESS - BEAM_STOCK_WIDTH,
            -VISUAL_OVERRUN,
            (POST_STOCK_THICKNESS - BEAM_STOCK_THICKNESS) / 2.0,
            // size
            SHORT_BEAM_UPPER_FRONT_TENON_WIDTH,
            POST_STOCK_WIDTH + (2.0 * VISUAL_OVERRUN),
            BEAM_STOCK_THICKNESS,
        )
    }

    fn rear_upper_short_beam_tenon_cutaway(&self) -> Cutaway {
        Cutaway::new(
            // position
            BASE_POST_HEIGHT - SIDE_ARM_THICKNESS - BEAM_STOCK_WIDTH,
            -VISUAL_OVERRUN,
            (POST_STOCK_THICKNESS - BEAM_STOCK_THICKNESS) / 2.0,
            // size
            SHORT_BEAM_UPPER_REAR_TENON_WIDTH,
            POST_STOCK_WIDTH + (2.0 * VISUAL_OVERRUN),
            BEAM_STOCK_THICKNESS,
        )
    }

    fn upper_long_beam_tenon_cutaway(&self) -> Cutaway {
        Cutaway::new(
            // position
            BASE_POST_HEIGHT - LONG_BEAM_TENON_WIDTH,
            (POST_STOCK_THICKNESS - BEAM_STOCK_THICKNESS) / 2.0,
            -VISUAL_OVERRUN,
            // size
            LONG_BEAM_TENON_WIDTH + VISUAL_OVERRUN,
            BEAM_STOCK_THICKNESS,
            POST_STOCK_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn add_front_cutaways(&self, parent: &mut ScadObject) {
        parent.add_child(self.lower_short_beam_tenon_cutaway().assemble());
        parent.add_child(self.lower_long_beam_tenon_cutaway().assemble());
        parent.add_child(self.front_upper_short_beam_tenon_cutaway().assemble())
    }

    fn add_rear_cutaways(&self, parent: &mut ScadObject) {
        parent.add_child(self.lower_short_beam_tenon_cutaway().assemble());
        parent.add_child(self.lower_long_beam_tenon_cutaway().assemble());
        parent.add_child(self.rear_upper_short_beam_tenon_cutaway().assemble());
        parent.add_child(self.upper_long_beam_tenon_cutaway().assemble())
    }
}

impl ObjectAssembler for Post {
    fn assemble(&self) -> ScadObject {
        // align back to center and orientate
        let mut pos = scad!(Translate(vec3(POST_STOCK_THICKNESS, 0.0, 0.0)));
        let mut rot = scad!(Rotate(-90.0, vec3(0.0, 1.0, 0.0)));

        // subtract the cutouts from the board
        let mut parent = scad!(Difference);
        parent.add_child(self.board.assemble());

        match self.loc {
            Loc::LeftFront => self.add_front_cutaways(&mut parent),
            Loc::LeftRear => self.add_rear_cutaways(&mut parent),
            Loc::RightFront => self.add_front_cutaways(&mut parent),
            Loc::RightRear => self.add_rear_cutaways(&mut parent),
        }

        // combine and return a parent container object
        rot.add_child(parent);
        pos.add_child(rot);
        pos
    }
}
