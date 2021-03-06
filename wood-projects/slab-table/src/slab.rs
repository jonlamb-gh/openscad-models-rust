use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::Board;
use scad::*;

use config::*;

qstruct!(Slab(color: Option<&'static str>) {
    board: Board = Board::new(
        SLAB_LENGTH,
        SLAB_WIDTH,
        SLAB_THICKNESS,
        color),
});

impl Slab {
    fn assemble_post_cutout(&self) -> ScadObject {
        scad!(Cube(vec3(
            POST_THICKNESS - (2.0 * POST_TENON_MAJOR_DEPTH),
            POST_WIDTH - (2.0 * POST_TENON_MINOR_DEPTH),
            SLAB_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )))
    }
}

impl ObjectAssembler for Slab {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        let post_a_offset = vec3(
            POST_TO_EDGE_DIST + POST_TENON_MAJOR_DEPTH,
            SLAB_WIDTH / 2.0 - POST_WIDTH / 2.0 + POST_TENON_MINOR_DEPTH,
            -VISUAL_OVERRUN,
        );
        let post_b_offset = vec3(
            SLAB_LENGTH - POST_TO_EDGE_DIST - POST_THICKNESS + POST_TENON_MAJOR_DEPTH,
            SLAB_WIDTH / 2.0 - POST_WIDTH / 2.0 + POST_TENON_MINOR_DEPTH,
            -VISUAL_OVERRUN,
        );

        scad!(Difference;{
            self.board.assemble(),
            scad!(Translate(post_a_offset);{
                self.assemble_post_cutout()
            }),
            scad!(Translate(post_b_offset);{
                self.assemble_post_cutout()
            }),
        })
    }
}
