use crate::assembler::ScadAssembler;
use crate::utils::{BoardDimensions, Centimeter, Color, Unit};
use scad::{scad, Cube, ScadObject};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Hash, Debug)]
pub struct Board<U: Unit + 'static = Centimeter> {
    dimensions: BoardDimensions<U>,
    color: Option<Color>,
}

impl<U: Unit + 'static> Board<U> {
    pub fn new(dimensions: BoardDimensions<U>) -> Self {
        Board {
            dimensions,
            color: None,
        }
    }

    pub fn with_color(dimensions: BoardDimensions<U>, color: Color) -> Self {
        Board {
            dimensions,
            color: Some(color),
        }
    }

    pub fn dimensions(&self) -> &BoardDimensions<U> {
        &self.dimensions
    }
}

impl<U: Unit + 'static> ScadAssembler for Board<U> {
    fn assemble(&self) -> ScadObject {
        scad!(Cube(self.dimensions().unitless_size()))
    }

    fn color(&self) -> Option<Color> {
        self.color
    }
}
