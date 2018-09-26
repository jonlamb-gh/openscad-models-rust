use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use config::*;
use foundation::Foundation;
use girders::Girders;
use posts::Posts;

qstruct!(House() {
    foundation: Foundation = Foundation::new(vec3(
        FOUNDATION_SIZE[0],
        FOUNDATION_SIZE[1],
        FOUNDATION_SIZE[2]),
        Some("Gainsboro")),
    girders: Girders = Girders::new(Some("SandyBrown")),
    posts: Posts = Posts::new(Some("SaddleBrown")),
});

impl ObjectAssembler for House {
    // TODO - containing volume?
    fn describe(&self) -> ObjectDescriptor {
        self.foundation.describe()
    }

    // TODO - positions
    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.foundation.assemble(),
            scad!(Translate(vec3(0.0, 0.0, 30.0));{
                self.girders.assemble(),
            }),
            scad!(Translate(vec3(0.0, 0.0, 30.0));{
                self.posts.assemble(),
            }),
        })
    }
}
