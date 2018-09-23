use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use super::na;

qstruct!(Foundation(size: na::Vector3<f32>) {
    size: na::Vector3<f32> = size,
});

impl ObjectAssembler for Foundation {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: self.size[0],
            width: self.size[1],
            thickness: self.size[2],
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
