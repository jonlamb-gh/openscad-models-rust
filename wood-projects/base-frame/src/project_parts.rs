use crate::{boards::*, config::*, joinery::*};
use parts::prelude::*;
use scad::ScadObject;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct LongFrameBoard(pub Board);

impl LongFrameBoard {
    pub fn new() -> Self {
        LongFrameBoard(long_frame_board())
    }
}

impl ScadAssembler for LongFrameBoard {
    fn assemble(&self) -> ScadObject {
        self.0.assemble_with(|obj, color| {
            let dims = self.0.dimensions();
            let obj = cut_bottom_ends(dims, obj);
            let obj = cut_slat_board_slots(dims, obj);
            color_or_render_root(obj, color)
        })
    }

    fn color(&self) -> Option<Color> {
        self.0.color()
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct ShortFrameBoard(pub Board);

impl ShortFrameBoard {
    pub fn new() -> Self {
        ShortFrameBoard(short_frame_board())
    }
}

impl ScadAssembler for ShortFrameBoard {
    fn assemble(&self) -> ScadObject {
        assert!(SHORT_FRAME_BOLT_OFFSET < ((SHORT_FRAME_BOARD_LENGTH / 2.0) - FRAME_BOARD_WIDTH));
        self.0.assemble_with(|obj, color| {
            let dims = self.0.dimensions();
            let obj = cut_top_ends(dims, obj);
            let obj = cut_bolt_holes(dims, obj);
            color_or_render_root(obj, color)
        })
    }

    fn color(&self) -> Option<Color> {
        self.0.color()
    }
}
