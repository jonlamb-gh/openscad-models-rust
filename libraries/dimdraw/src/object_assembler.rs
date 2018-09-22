use scad::*;

pub struct ObjectDescriptor {
    pub length: f32,
    pub width: f32,
    pub thickness: f32,
}

pub trait ObjectAssembler {
    fn describe(&self) -> ObjectDescriptor;

    fn assemble(&self) -> ScadObject;

    fn assemble_at(&self, x: f32, y: f32, z: f32) -> ScadObject {
        scad!(Translate(vec3(x, y, z));{
            self.assemble()
        })
    }
}
