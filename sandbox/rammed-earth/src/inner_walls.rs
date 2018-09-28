use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Wall;
use scad::*;

use config::*;

qstruct!(InnerWalls(color: Option<&'static str>) {
    l8: Wall = Wall::new(
        INNER_WALL_L8_LENGTH,
        INNER_WALL_WIDTH,
        INNER_WALL_THICKNESS,
        color),
    l10: Wall = Wall::new(
        INNER_WALL_L10_LENGTH,
        INNER_WALL_WIDTH,
        INNER_WALL_THICKNESS,
        color),
});

impl InnerWalls {
    fn assemble_rows(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // row C/D
        let x_offset = ft_to_cm(32.0);
        let y_offset = ft_to_cm(20.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10.assemble_cxaligned()
        }));

        parent
    }
}

impl ObjectAssembler for InnerWalls {
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
            self.assemble_rows(),
        })
    }
}
