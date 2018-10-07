use dimdraw::{DrawingAssembler, DrawingParams, ObjectAssembler, ObjectDescriptor};
use scad::*;

use config::*;
use post::Post;
use slab::Slab;

qstruct!(SlabTable() {
    slab: Slab = Slab::new(Some("SandyBrown")),
    post: Post = Post::new(Some("Peru")),
});

impl DrawingAssembler for SlabTable {
    fn drawing_params(&self) -> DrawingParams {
        let delta = 30.0;
        DrawingParams {
            doc_scale: 40.0,
            show_frame: true,
            doc_height: POST_LENGTH + 5.0,
            top_left_view_pos: vec3(-SLAB_LENGTH - delta, delta, 0.0),
            top_right_view_pos: vec3(delta * 2.5, delta, 0.0),
            bottom_left_view_pos: vec3(-SLAB_LENGTH - delta, -SLAB_WIDTH - delta, 0.0),
            bottom_right_view_pos: vec3(delta, -POST_LENGTH - delta, 0.0),
        }
    }
}

impl ObjectAssembler for SlabTable {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: SLAB_LENGTH,
            width: SLAB_WIDTH,
            thickness: POST_LENGTH,
        }
    }

    fn assemble(&self) -> ScadObject {
        let post_a_offset = vec3(POST_TO_EDGE_DIST, SLAB_WIDTH / 2.0 - POST_WIDTH / 2.0, 0.0);
        let post_b_offset = vec3(
            SLAB_LENGTH - POST_TO_EDGE_DIST - POST_THICKNESS,
            SLAB_WIDTH / 2.0 - POST_WIDTH / 2.0,
            0.0,
        );
        let slab_offset = vec3(0.0, 0.0, POST_LENGTH - SLAB_THICKNESS - TENON_OVERRUN);

        scad!(Union;{
            scad!(Translate(slab_offset);{
                self.slab.assemble(),
            }),
            scad!(Translate(post_a_offset);{
                self.post.assemble_aligned(),
            }),
            scad!(Translate(post_b_offset);{
                self.post.assemble_aligned(),
            }),
        })
    }
}
