// TODO - move constants out, 8' X 8' module as an object?
use dimdraw::{some_color, ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use scad::*;

use config::*;

const OUTER_WALL_2X_THICKNESS: f32 = 2.0 * OUTER_WALL_THICKNESS;

// 48' X 32' plus wall thicknesses
pub const FOUNDATION_LENGTH: f32 = 1463.04 + OUTER_WALL_2X_THICKNESS;
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
            scad!(Translate(vec3(ft_to_cm(40.0) + OUTER_WALL_THICKNESS, ft_to_cm(8.0), 0.0));{
                self.assemble_minor(),
            }),
            self.assemble_outer_wall_footing(),
        })
    }

    fn assemble_major(&self) -> ScadObject {
        let mut parent = scad!(Union);

        for row in 0..4 {
            let y_offset = row as f32 * ft_to_cm(8.0) + OUTER_WALL_THICKNESS;
            for col in 0..5 {
                let x_offset = col as f32 * ft_to_cm(8.0) + OUTER_WALL_THICKNESS;
                parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                        self.assemble_module()
                    }));
            }
        }

        parent
    }

    fn assemble_minor(&self) -> ScadObject {
        let mut parent = scad!(Union);

        let x_offset = 0.0;
        for row in 0..3 {
            let y_offset = row as f32 * ft_to_cm(8.0) + OUTER_WALL_THICKNESS;
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.assemble_module()
            }));
        }

        parent
    }

    fn assemble_outer_wall_footing(&self) -> ScadObject {
        scad!(Union;{
            scad!(Cube(vec3(ft_to_cm(44.0), OUTER_WALL_THICKNESS, FOUNDATION_THICKNESS))),
            scad!(Translate(vec3(0.0, OUTER_WALL_THICKNESS, 0.0));{
                scad!(Cube(vec3(OUTER_WALL_THICKNESS, ft_to_cm(32.0), FOUNDATION_THICKNESS))),
            }),
            scad!(Translate(vec3(ft_to_cm(50.0), ft_to_cm(8.0), 0.0));{
                scad!(Cube(vec3(OUTER_WALL_THICKNESS, ft_to_cm(28.0), FOUNDATION_THICKNESS))),
            }),
            scad!(Translate(vec3(ft_to_cm(42.0), OUTER_WALL_THICKNESS, 0.0));{
                scad!(Cube(vec3(OUTER_WALL_THICKNESS, ft_to_cm(8.0), FOUNDATION_THICKNESS))),
            }),
            scad!(Translate(vec3(ft_to_cm(44.0), ft_to_cm(8.0), 0.0));{
                scad!(Cube(vec3(ft_to_cm(6.0), OUTER_WALL_THICKNESS, FOUNDATION_THICKNESS))),
            }),
            scad!(Translate(vec3(0.0, ft_to_cm(34.0), 0.0));{
                scad!(Cube(vec3(ft_to_cm(50.0), OUTER_WALL_THICKNESS, FOUNDATION_THICKNESS))),
            })
        })
    }

    fn assemble_module(&self) -> ScadObject {
        scad!(Cube(vec3(
            ft_to_cm(8.0),
            ft_to_cm(8.0),
            FOUNDATION_THICKNESS
        )))
    }
}
