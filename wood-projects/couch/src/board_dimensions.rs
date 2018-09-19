use scad::*;

use object_assembler::ObjectAssembler;

pub struct BoardDimensions {
    // size[0] is always with the grain
    // size[1] is against the grain
    // size[2] is the board thickness
    size: [f32; 3],
}

impl BoardDimensions {
    pub fn new(length: f32, width: f32, thickness: f32) -> Self {
        assert!(length > 0.0);
        assert!(width > 0.0);
        assert!(thickness > 0.0);
        Self {
            size: [length, width, thickness],
        }
    }

    /*
    pub fn size(&self) -> &[f32; 3] {
        &self.size
    }
    */

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

impl ObjectAssembler for BoardDimensions {
    fn assemble(&self) -> ScadObject {
        scad!(Cube(vec3(self.length(), self.width(), self.thickness(),)))
    }
}
