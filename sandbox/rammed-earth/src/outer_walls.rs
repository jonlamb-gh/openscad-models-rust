use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Wall;
use scad::*;

use config::*;

qstruct!(OuterWalls(color: Option<&'static str>) {
    l8: Wall = Wall::new(
        OUTER_WALL_L8_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color),
});

impl ObjectAssembler for OuterWalls {
    // TODO - containing volume?
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: ft_to_cm(44.0),
            width: ft_to_cm(32.0),
            thickness: ft_to_cm(50.0),
        }
    }

    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.l8.assemble_yaligned(),
        })
    }
}
