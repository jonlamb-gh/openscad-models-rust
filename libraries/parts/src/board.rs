use dimdraw::{some_color, ObjectAssembler, ObjectDescriptor};
use scad::*;

use board_dimensions::BoardDimensions;

pub struct Board {
    dimensions: BoardDimensions,
    color: Option<String>,
}

impl Board {
    pub fn new(length: f32, width: f32, thickness: f32, color: Option<&'static str>) -> Self {
        Self {
            dimensions: BoardDimensions::new(length, width, thickness),
            color: some_color(color),
        }
    }

    pub fn from_array(size: &[f32; 3], color: Option<&'static str>) -> Self {
        Self::new(size[0], size[1], size[2], color)
    }

    pub fn dims(&self) -> &BoardDimensions {
        &self.dimensions
    }

    pub fn length(&self) -> f32 {
        self.dimensions.length()
    }

    pub fn width(&self) -> f32 {
        self.dimensions.width()
    }

    pub fn thickness(&self) -> f32 {
        self.dimensions.thickness()
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

    fn object_color(&self) -> Option<ScadObject> {
        if let Some(ref c) = self.color {
            Some(scad!(NamedColor(c.to_string())))
        } else {
            None
        }
    }

    fn assemble(&self) -> ScadObject {
        if let Some(mut c) = self.object_color() {
            c.add_child(scad!(Cube(vec3(
                self.dimensions.length(),
                self.dimensions.width(),
                self.dimensions.thickness(),
            ))));
            c
        } else {
            scad!(Cube(vec3(
                self.dimensions.length(),
                self.dimensions.width(),
                self.dimensions.thickness(),
            )))
        }
    }
}
