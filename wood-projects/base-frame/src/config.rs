use parts::prelude::*;

pub const EXPLODED_VIEW_OFFSET: Centimeter = Centimeter::new(10.0);

pub const FRAME_BOARD_WIDTH: Centimeter = Centimeter::new(3.8);
pub const FRAME_BOARD_THICKNESS: Centimeter = Centimeter::new(3.8);

pub const LONG_FRAME_BOARD_LENGTH: Centimeter = Centimeter::new(129.3);
pub const LONG_FRAME_BOARD_COLOR: Color = Color::SaddleBrown;

pub const SHORT_FRAME_BOARD_LENGTH: Centimeter = Centimeter::new(68.0);
pub const SHORT_FRAME_BOARD_COLOR: Color = Color::Sienna;

pub const NUM_SLAT_BOARDS: usize = 8;
pub const SLAT_BOARD_SEP_DISTANCE: Centimeter = Centimeter::new(6.0);
pub const SLAT_BOARD_LENGTH: Centimeter = SHORT_FRAME_BOARD_LENGTH;
pub const SLAT_BOARD_WIDTH: Centimeter = Centimeter::new(8.9);
pub const SLAT_BOARD_THICKNESS: Centimeter = Centimeter::new(1.8);
pub const SLAT_BOARD_COLOR: Color = Color::SandyBrown;

pub const BOLT_HOLE_DIAMETER: Centimeter = Centimeter::new(0.635);

/// Offset from center
pub const SHORT_FRAME_BOLT_OFFSET: Centimeter = Centimeter::new(25.0);
