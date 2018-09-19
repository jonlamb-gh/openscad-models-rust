use scad::*;

use config::*;
use long_beam::LongBeam;
use lower_short_beam::LowerShortBeam;
use object_assembler::ObjectAssembler;
use post::{Loc, Post};

qstruct!(Couch() {
    post_lf: Post = Post::new(Loc::LeftFront),
    post_lr: Post = Post::new(Loc::LeftRear),
    post_rf: Post = Post::new(Loc::RightFront),
    post_rr: Post = Post::new(Loc::RightRear),

    long_beam: LongBeam = LongBeam::new(),
    lower_short_beam: LowerShortBeam = LowerShortBeam::new(),
});

impl Couch {
    fn assemble_posts(&self) -> ScadObject {
        let dx = POST_STOCK_THICKNESS + BASE_POST_TO_POST_LENGTH;
        let dy = POST_STOCK_WIDTH + BASE_POST_TO_POST_DEPTH;

        scad!(Union;{
            self.post_lf.assemble(),
            scad!(Translate(vec3(dx, 0.0, 0.0));{
                self.post_rf.assemble(),
            }),
            scad!(Translate(vec3(0.0, dy, 0.0));{
                self.post_lr.assemble(),
            }),
            scad!(Translate(vec3(dx, dy, 0.0));{
                self.post_rr.assemble(),
            }),
        })
    }

    fn assemble_base_beams(&self) -> ScadObject {
        assert_eq!(POST_STOCK_THICKNESS, POST_STOCK_WIDTH);

        let height_offset = BASE_HEIGHT - BEAM_STOCK_WIDTH;
        let side_height_offset = BASE_HEIGHT;
        let depth_offset = (POST_STOCK_THICKNESS - BEAM_STOCK_THICKNESS) / 2.0;

        let front_long_beam_pos = vec3(-TENON_OVERRUN, depth_offset, height_offset);

        let rear_long_beam_pos = vec3(
            -TENON_OVERRUN,
            depth_offset + POST_STOCK_WIDTH + BASE_POST_TO_POST_DEPTH,
            height_offset,
        );

        let left_lower_short_beam_pos = vec3(depth_offset, -TENON_OVERRUN, side_height_offset);

        let right_lower_short_beam_pos = vec3(
            depth_offset + POST_STOCK_THICKNESS + BASE_POST_TO_POST_LENGTH,
            -TENON_OVERRUN,
            side_height_offset,
        );

        scad!(Union;{
            scad!(Translate(front_long_beam_pos);{
                self.long_beam.assemble(),
            }),
            scad!(Translate(rear_long_beam_pos);{
                self.long_beam.assemble(),
            }),
            scad!(Translate(left_lower_short_beam_pos);{
                self.lower_short_beam.assemble(),
            }),
            scad!(Translate(right_lower_short_beam_pos);{
                self.lower_short_beam.assemble(),
            }),
        })
    }
}

impl ObjectAssembler for Couch {
    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.assemble_posts(),
            self.assemble_base_beams()
        })
    }
}
