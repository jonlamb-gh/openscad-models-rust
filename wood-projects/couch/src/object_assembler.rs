use scad::*;

pub trait ObjectAssembler {
    fn assemble(&self) -> ScadObject;

    fn assemble_at(&self, x: f32, y: f32, z: f32) -> ScadObject {
        scad!(Translate(vec3(x, y, z));{
            self.assemble()
        })
    }
}
