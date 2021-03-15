use parts::prelude::*;

pub const FRAME_BOARD_WIDTH: Centimeter = Centimeter::new(5.0);
pub const FRAME_BOARD_THICKNESS: Centimeter = Centimeter::new(5.0);

pub const LONG_FRAME_BOARD_LENGTH: Centimeter = Centimeter::new(129.3);
pub const LONG_FRAME_BOARD_COLOR: Color = Color::SaddleBrown;

pub const SHORT_FRAME_BOARD_LENGTH: Centimeter = Centimeter::new(68.0);
pub const SHORT_FRAME_BOARD_COLOR: Color = Color::Sienna;

pub const NUM_SLAT_BOARDS: usize = 8;
pub const SLAT_BOARD_SEP_DISTANCE: Centimeter = Centimeter::new(6.2);
pub const SLAT_BOARD_LENGTH: Centimeter = SHORT_FRAME_BOARD_LENGTH;
pub const SLAT_BOARD_WIDTH: Centimeter = Centimeter::new(8.0);
pub const SLAT_BOARD_THICKNESS: Centimeter = Centimeter::new(1.5);
pub const SLAT_BOARD_COLOR: Color = Color::SandyBrown;
