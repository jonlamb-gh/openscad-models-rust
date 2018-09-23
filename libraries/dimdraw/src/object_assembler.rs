use scad::*;

use constants::vec3z;

pub struct ObjectDescriptor {
    pub length: f32,
    pub width: f32,
    pub thickness: f32,
}

pub trait ObjectAssembler {
    fn describe(&self) -> ObjectDescriptor;

    fn has_color(&self) -> bool {
        false
    }

    // return Option<> instead of has_color?
    fn object_color(&self) -> ScadObject {
        scad!(Color(vec3z()))
    }

    // TODO - apply color here? currenty yes
    fn assemble(&self) -> ScadObject;

    fn assemble_at(&self, x: f32, y: f32, z: f32) -> ScadObject {
        scad!(Translate(vec3(x, y, z));{
            self.assemble()
        })
    }
}
