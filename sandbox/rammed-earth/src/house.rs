use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use config::*;
use doors::Doors;
use foundation::Foundation;
use inner_walls::InnerWalls;
use outer_walls::OuterWalls;
use roof::Roof;

qstruct!(House() {
    foundation: Foundation = Foundation::new(Some("Silver")),
    outer_walls: OuterWalls = OuterWalls::new(Some("SaddleBrown"), Some("Peru")),
    inner_walls: InnerWalls = InnerWalls::new(Some("SaddleBrown"), Some("Peru")),
    doors: Doors = Doors::new(Some("Sienna")),
    roof: Roof = Roof::new(Some("SandyBrown")),
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
            scad!(Translate(vec3(0.0, 0.0, FOUNDATION_THICKNESS));{
                self.doors.assemble(),
            }),
            scad!(Translate(vec3(0.0, 0.0, FOUNDATION_THICKNESS + OUTER_WALL_WIDTH));{
                self.roof.assemble(),
            }),
        })
    }
}
