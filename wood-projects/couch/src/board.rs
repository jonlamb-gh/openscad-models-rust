use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use board_dimensions::BoardDimensions;

pub struct Board {
    dimensions: BoardDimensions,
}

impl Board {
    pub fn new(length: f32, width: f32, thickness: f32) -> Self {
        Self {
            dimensions: BoardDimensions::new(length, width, thickness),
        }
    }

    pub fn from_array(size: &[f32; 3]) -> Self {
        Self::new(size[0], size[1], size[2])
    }

    pub fn dims(&self) -> &BoardDimensions {
        &self.dimensions
    }
}

impl ObjectAssembler for Board {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: self.dimensions.length(),
            width: self.dimensions.width(),
            thickness: self.dimensions.thickness(),
        }
    }

    fn assemble(&self) -> ScadObject {
        scad!(Cube(vec3(
            self.dimensions.length(),
            self.dimensions.width(),
            self.dimensions.thickness(),
        )))
    }
}
