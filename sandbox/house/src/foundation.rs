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

    fn has_color(&self) -> bool {
        if let Some(_) = self.color {
            true
        } else {
            false
        }
    }

    fn object_color(&self) -> ScadObject {
        if let Some(ref c) = self.color {
            scad!(NamedColor(c.to_string()))
        } else {
            scad!(Color(vec3(0.0, 0.0, 0.0)))
        }
    }

    fn assemble(&self) -> ScadObject {
        if self.has_color() {
            let mut color_obj = self.object_color();
            color_obj.add_child(scad!(Cube(vec3(self.size[0], self.size[1], self.size[2],))));
            color_obj
        } else {
            scad!(Cube(vec3(self.size[0], self.size[1], self.size[2],)))
        }
    }
}
