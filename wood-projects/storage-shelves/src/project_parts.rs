use crate::{boards::*, joinery::*};
use derive_more::Into;
use parts::prelude::*;
use scad::ScadObject;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Into)]
pub struct FullShelfBoard(pub Board<Inch>);

impl Default for FullShelfBoard {
    fn default() -> Self {
        FullShelfBoard(long_board())
    }
}

impl ScadAssembler for FullShelfBoard {
    fn assemble(&self) -> ScadObject {
        self.0.assemble_with(|obj, color| {
            let obj = cut_full_shelf_long_board_screw_holes(obj);
            color_or_render_root(obj, color)
        })
    }

    fn color(&self) -> Option<Color> {
        self.0.color()
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Into)]
pub struct HalfShelfBoard(pub Board<Inch>);

impl Default for HalfShelfBoard {
    fn default() -> Self {
        HalfShelfBoard(med_board())
    }
}

impl ScadAssembler for HalfShelfBoard {
    fn assemble(&self) -> ScadObject {
        self.0.assemble_with(|obj, color| {
            let obj = cut_half_shelf_med_board_screw_holes(obj);
            color_or_render_root(obj, color)
        })
    }

    fn color(&self) -> Option<Color> {
        self.0.color()
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Into)]
pub struct ShortBoard(pub Board<Inch>);

impl Default for ShortBoard {
    fn default() -> Self {
        ShortBoard(short_board())
    }
}

impl ScadAssembler for ShortBoard {
    fn assemble(&self) -> ScadObject {
        self.0.assemble_with(color_or_render_root)
    }

    fn color(&self) -> Option<Color> {
        self.0.color()
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum VerticalSupportBoardScrewPattern {
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct VerticalSupportBoard(pub Board<Inch>, pub VerticalSupportBoardScrewPattern);

impl VerticalSupportBoard {
    pub fn new(p: VerticalSupportBoardScrewPattern) -> Self {
        VerticalSupportBoard(long_board(), p)
    }
}

impl ScadAssembler for VerticalSupportBoard {
    fn assemble(&self) -> ScadObject {
        use VerticalSupportBoardScrewPattern::*;
        self.0.assemble_with(|obj, color| {
            let obj = match self.1 {
                A => cut_vert_support_long_board_screw_holes_a(obj),
                B => cut_vert_support_long_board_screw_holes_b(obj),
                C => cut_vert_support_long_board_screw_holes_c(obj),
                D => cut_vert_support_long_board_screw_holes_d(obj),
            };
            color_or_render_root(obj, color)
        })
    }

    fn color(&self) -> Option<Color> {
        self.0.color()
    }
}
