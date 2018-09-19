use scad::*;

use config::*;
use object_assembler::ObjectAssembler;
use post::{Loc, Post};

qstruct!(Couch() {
    post_lf: Post = Post::new(Loc::LeftFront),
    post_lr: Post = Post::new(Loc::LeftRear),
    post_rf: Post = Post::new(Loc::RightFront),
    post_rr: Post = Post::new(Loc::RightRear),
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
}

impl ObjectAssembler for Couch {
    fn assemble(&self) -> ScadObject {
        self.assemble_posts()
    }
}
