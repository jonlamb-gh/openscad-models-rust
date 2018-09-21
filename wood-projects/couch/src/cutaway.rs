use scad::*;

use dimdraw::ObjectAssembler;

// TODO - use vector3 or some other proper types
qstruct!(Cutaway(x: f32, y: f32, z: f32, sx: f32, sy: f32, sz: f32) {
    // position
    x: f32 = x,
    y: f32 = y,
    z: f32 = z,
    // size
    sx: f32 = sx,
    sy: f32 = sy,
    sz: f32 = sz,
});

impl ObjectAssembler for Cutaway {
    fn assemble(&self) -> ScadObject {
        assert!(self.sx > 0.0);
        assert!(self.sy > 0.0);
        assert!(self.sz > 0.0);

        scad!(Translate(vec3(self.x, self.y, self.z));{
            scad!(Cube(vec3(self.sx, self.sy, self.sz))),
        })
    }
}
