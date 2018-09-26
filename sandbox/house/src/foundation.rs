use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use super::na;

qstruct!(Foundation(size: na::Vector3<f32>, color: Option<&'static str>) {
    size: na::Vector3<f32> = size,
    color: Option<String> = if let Some(c) = color { Some(c.to_string()) } else { None },
});

impl ObjectAssembler for Foundation {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: self.size[0],
            width: self.size[1],
            thickness: self.size[2],
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
            c.add_child(scad!(Cube(vec3(self.size[0], self.size[1], self.size[2],))));
            c
        } else {
            scad!(Cube(vec3(self.size[0], self.size[1], self.size[2],)))
        }
    }
}
