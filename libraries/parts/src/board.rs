use crate::utils::{BoardDimensions, Color};
use scad::{scad, Cube, ScadObject};
use scad_assembler::ScadAssembler;

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
        let obj = scad!(Cube(self.dimensions().unitless_size()));
        match &self.color {
            None => obj,
            Some(c) => {
                let mut parent = c.to_scad();
                parent.add_child(obj);
                parent
            }
        }
    }
}
