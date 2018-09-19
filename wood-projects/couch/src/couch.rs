use scad::*;

use object_assembler::ObjectAssembler;
use post::{Loc, Post};

qstruct!(Couch() {
    post: Post = Post::new(Loc::LeftFront),
});

impl ObjectAssembler for Couch {
    fn assemble(&self) -> ScadObject {
        self.post.assemble()
    }
}
