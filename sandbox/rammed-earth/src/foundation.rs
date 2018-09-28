use dimdraw::{some_color, ObjectAssembler, ObjectDescriptor};
use scad::*;
use parts::common_functions::*;

use config::*;

qstruct!(Foundation(color: Option<&'static str>) {
    color: Option<String> = some_color(color),
});

impl ObjectAssembler for Foundation {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: ft_to_cm(44.0),
            width: ft_to_cm(32.0),
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
            scad!(Translate(vec3(ft_to_cm(32.0), ft_to_cm(8.0), 0.0));{
                self.assemble_minor(),
            }),
        })
    }

    fn assemble_major(&self) -> ScadObject {
        scad!(Cube(vec3(
            ft_to_cm(32.0),
            ft_to_cm(32.0),
            FOUNDATION_THICKNESS)))
    }

    fn assemble_minor(&self) -> ScadObject {
        scad!(Cube(vec3(
            ft_to_cm(12.0),
            ft_to_cm(24.0),
            FOUNDATION_THICKNESS)))
    }
}
