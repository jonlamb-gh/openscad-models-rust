use dimdraw::{ObjectAssembler, ObjectDescriptor};
use nalgebra::Vector3;
use scad::{qstruct, scad, Cube, ScadObject, Translate};

qstruct!(Cutaway(pos: Vector3<f32>, size: Vector3<f32>) {
    pos: Vector3<f32> = pos,
    size: Vector3<f32> = size,
});

impl Cutaway {
    pub fn from_parts(x: f32, y: f32, z: f32, sx: f32, sy: f32, sz: f32) -> Self {
        Self {
            pos: Vector3::new(x, y, z),
            size: Vector3::new(sx, sy, sz),
        }
    }
}

impl ObjectAssembler for Cutaway {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: self.size.x,
            width: self.size.y,
            thickness: self.size.z,
        }
    }

    fn assemble(&self) -> ScadObject {
        assert!(self.size.x > 0.0);
        assert!(self.size.y > 0.0);
        assert!(self.size.z > 0.0);

        scad!(Translate(self.pos);{
            scad!(Cube(self.size)),
        })
    }
}
