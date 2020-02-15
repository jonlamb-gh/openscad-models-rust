use crate::utils::{BoardDimensions, Color};
use scad::{scad, Cube, ScadObject};
use scad_assembler::ScadAssembler;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Board {
    dimensions: BoardDimensions,
    color: Option<Color>,
}

impl Board {
    pub fn new(dimensions: BoardDimensions, color: Option<Color>) -> Self {
        Board { dimensions, color }
    }

    pub fn dimensions(&self) -> &BoardDimensions {
        &self.dimensions
    }
}

impl ScadAssembler for Board {
    fn assemble(&self) -> ScadObject {
        let obj = scad!(Cube(*self.dimensions().size()));
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
