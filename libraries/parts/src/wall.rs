use dimdraw::{some_color, ObjectAssembler, ObjectDescriptor};
use scad::*;

use common_functions::*;

pub struct Wall {
    size: [f32; 3],
    color: Option<String>,
}

impl Wall {
    pub fn new(length: f32, width: f32, thickness: f32, color: Option<&'static str>) -> Self {
        Self {
            size: [length, width, thickness],
            color: some_color(color),
        }
    }

    pub fn from_array(size: &[f32; 3], color: Option<&'static str>) -> Self {
        Self::new(size[0], size[1], size[2], color)
    }

    pub fn length(&self) -> f32 {
        self.size[0]
    }

    pub fn width(&self) -> f32 {
        self.size[1]
    }

    pub fn thickness(&self) -> f32 {
        self.size[2]
    }
}

impl ObjectAssembler for Wall {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: self.length(),
            width: self.width(),
            thickness: self.thickness(),
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
            c.add_child(scad!(Cube(vec3(
                self.length(),
                self.width(),
                self.thickness(),
            ))));
            c
        } else {
            scad!(Cube(vec3(self.length(), self.width(), self.thickness(),)))
        }
    }
}

impl Wall {
    pub fn assemble_xaligned(&self) -> ScadObject {
        scad!(Translate(vec3(0.0, self.thickness(), 0.0));{
            scad!(Rotate(90.0, x_axis());{
                self.assemble()
            })
        })
    }

    pub fn assemble_yaligned(&self) -> ScadObject {
        scad!(Rotate(90.0, z_axis());{
            scad!(Rotate(90.0, x_axis());{
                self.assemble()
            })
        })
    }
}
