use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use config::*;
use foundation::Foundation;
use outer_walls::OuterWalls;

qstruct!(House() {
    foundation: Foundation = Foundation::new(vec3(
        FOUNDATION_SIZE[0],
        FOUNDATION_SIZE[1],
        FOUNDATION_SIZE[2]),
        Some("Gainsboro")),
    outer_walls: OuterWalls = OuterWalls::new(Some("SaddleBrown")),
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
            self.outer_walls.assemble(),
        })
    }
}
