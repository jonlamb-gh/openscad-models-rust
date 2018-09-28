use dimdraw::{some_color, ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use scad::*;

use config::*;

const OUTER_WALL_2X_THICKNESS: f32 = 2.0 * OUTER_WALL_THICKNESS;

// 44' plus wall thicknesses
pub const FOUNDATION_LENGTH: f32 = 1341.12 + OUTER_WALL_2X_THICKNESS;
pub const FOUNDATION_WIDTH: f32 = 975.36 + OUTER_WALL_2X_THICKNESS;

qstruct!(Foundation(color: Option<&'static str>) {
    color: Option<String> = some_color(color),
});

impl ObjectAssembler for Foundation {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: FOUNDATION_LENGTH,
            width: FOUNDATION_WIDTH,
            thickness: FOUNDATION_THICKNESS,
        }
    }

    fn object_color(&self) -> Option<ScadObject> {
        if let Some(ref c) = self.color {
            Some(scad!(NamedColor(c.to_string())))
        } else {
            None
        }
    }

    fn assemble(&self) -> ScadObject {
        if let Some(mut c) = self.object_color() {
            c.add_child(self.assemble_both());
            c
        } else {
            self.assemble_both()
        }
    }
}

impl Foundation {
    fn assemble_both(&self) -> ScadObject {
        scad!(Union;{
            self.assemble_major(),
            scad!(Translate(vec3(ft_to_cm(32.0) + OUTER_WALL_2X_THICKNESS, ft_to_cm(8.0), 0.0));{
                self.assemble_minor(),
            }),
        })
    }

    fn assemble_major(&self) -> ScadObject {
        scad!(Cube(vec3(
            ft_to_cm(32.0) + OUTER_WALL_2X_THICKNESS,
            ft_to_cm(32.0) + OUTER_WALL_2X_THICKNESS,
            FOUNDATION_THICKNESS
        )))
    }

    fn assemble_minor(&self) -> ScadObject {
        scad!(Cube(vec3(
            ft_to_cm(12.0) + OUTER_WALL_THICKNESS,
            ft_to_cm(24.0) + OUTER_WALL_2X_THICKNESS,
            FOUNDATION_THICKNESS
        )))
    }
}
