use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use board_dimensions::BoardDimensions;

pub struct Board {
    dimensions: BoardDimensions,
    color: Option<String>,
}

impl Board {
    pub fn new(length: f32, width: f32, thickness: f32, color: Option<&'static str>) -> Self {
        let mc = if let Some(c) = color {
            Some(c.to_string())
        } else {
            None
        };

        Self {
            dimensions: BoardDimensions::new(length, width, thickness),
            color: mc,
        }
    }

    pub fn from_array(size: &[f32; 3], color: Option<&'static str>) -> Self {
        Self::new(size[0], size[1], size[2], color)
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
            color_obj.add_child(scad!(Cube(vec3(
                self.dimensions.length(),
                self.dimensions.width(),
                self.dimensions.thickness(),
            ))));
            color_obj
        } else {
            scad!(Cube(vec3(
                self.dimensions.length(),
                self.dimensions.width(),
                self.dimensions.thickness(),
            )))
        }
    }
}
