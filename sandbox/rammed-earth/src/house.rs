use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use config::*;
use foundation::Foundation;
use inner_walls::InnerWalls;
use outer_walls::OuterWalls;

qstruct!(House() {
    foundation: Foundation = Foundation::new(Some("Gainsboro")),
    outer_walls: OuterWalls = OuterWalls::new(Some("SandyBrown"), Some("SaddleBrown")),
    inner_walls: InnerWalls = InnerWalls::new(Some("SaddleBrown")),
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
            scad!(Translate(vec3(0.0, 0.0, FOUNDATION_THICKNESS));{
                self.outer_walls.assemble(),
            }),
            scad!(Translate(vec3(0.0, 0.0, FOUNDATION_THICKNESS));{
                self.inner_walls.assemble(),
            }),
        })
    }
}
