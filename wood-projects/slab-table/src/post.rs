use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Board;
use scad::*;

use config::*;

qstruct!(Post(color: Option<&'static str>) {
    board: Board = Board::new(
        POST_LENGTH,
        POST_WIDTH,
        POST_THICKNESS,
        color),
});

impl Post {
    pub fn assemble_aligned(&self) -> ScadObject {
        scad!(Translate(vec3(self.board.thickness(), 0.0, 0.0));{
            scad!(Rotate(-90.0, y_axis());{
                self.assemble()
            })
        })
    }

    fn assemble_major_cutout(&self) -> ScadObject {
        scad!(Cube(vec3(
            SLAB_THICKNESS + TENON_OVERRUN + VISUAL_OVERRUN,
            POST_WIDTH + (2.0 * VISUAL_OVERRUN),
            POST_TENON_MAJOR_DEPTH + VISUAL_OVERRUN,
        )))
    }

    fn assemble_minor_cutout(&self) -> ScadObject {
        scad!(Cube(vec3(
            SLAB_THICKNESS + TENON_OVERRUN + VISUAL_OVERRUN,
            POST_TENON_MINOR_DEPTH + VISUAL_OVERRUN,
            POST_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )))
    }
}

impl ObjectAssembler for Post {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        let cutout_a_offset = vec3(
            self.board.length() - SLAB_THICKNESS - TENON_OVERRUN,
            -VISUAL_OVERRUN,
            -VISUAL_OVERRUN,
        );
        let cutout_b_offset = vec3(
            self.board.length() - SLAB_THICKNESS - TENON_OVERRUN,
            -VISUAL_OVERRUN,
            self.board.thickness() - POST_TENON_MAJOR_DEPTH,
        );
        let cutout_c_offset = vec3(
            self.board.length() - SLAB_THICKNESS - TENON_OVERRUN,
            -VISUAL_OVERRUN,
            -VISUAL_OVERRUN,
        );
        let cutout_d_offset = vec3(
            self.board.length() - SLAB_THICKNESS - TENON_OVERRUN,
            self.board.width() - POST_TENON_MINOR_DEPTH,
            -VISUAL_OVERRUN,
        );

        scad!(Difference;{
            self.board.assemble(),
            scad!(Translate(cutout_a_offset);{
                self.assemble_major_cutout()
            }),
            scad!(Translate(cutout_b_offset);{
                self.assemble_major_cutout()
            }),
            scad!(Translate(cutout_c_offset);{
                self.assemble_minor_cutout()
            }),
            scad!(Translate(cutout_d_offset);{
                self.assemble_minor_cutout()
            }),
        })
    }
}
