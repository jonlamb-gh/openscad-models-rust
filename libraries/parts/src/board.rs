use crate::assembler::ScadAssembler;
use crate::utils::{BoardDimensions, Color};
use scad::{scad, Cube, ScadObject};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Board {
    dimensions: BoardDimensions,
    color: Option<Color>,
}

impl Board {
    pub fn new(dimensions: BoardDimensions) -> Self {
        Board {
            dimensions,
            color: None,
        }
    }

    pub fn with_color(dimensions: BoardDimensions, color: Color) -> Self {
        Board {
            dimensions,
            color: Some(color),
        }
    }

    pub fn dimensions(&self) -> &BoardDimensions {
        &self.dimensions
    }
}

impl ScadAssembler for Board {
    fn assemble(&self) -> ScadObject {
        scad!(Cube(self.dimensions().unitless_size()))
    }

    fn color(&self) -> Option<Color> {
        self.color
    }
}
