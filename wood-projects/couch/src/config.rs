// TODO - fix this up
// units: centimeter
#![allow(dead_code)]

use common_functions::*;

pub const VISUAL_OVERRUN: f32 = 0.1;

pub const BASE_POST_TO_POST_LENGTH: f32 = ft_to_cm(10.0);
pub const BASE_POST_TO_POST_DEPTH: f32 = ft_to_cm(3.0);
pub const BASE_POST_HEIGHT: f32 = ft_to_cm(2.5);
pub const BASE_HEIGHT: f32 = in_to_cm(12.0);

pub const TENON_OVERRUN: f32 = 1.0;
pub const PLANK_OVERRUN: f32 = 1.0;

pub const POST_STOCK_WIDTH: f32 = in_to_cm(6.0);
pub const POST_STOCK_THICKNESS: f32 = in_to_cm(6.0);

pub const BEAM_STOCK_WIDTH: f32 = in_to_cm(6.0);
pub const BEAM_STOCK_THICKNESS: f32 = in_to_cm(2.0);

pub const SIDE_ARM_WIDTH: f32 = POST_STOCK_WIDTH;
pub const SIDE_ARM_THICKNESS: f32 = in_to_cm(2.0);
pub const SIDE_ARM_TENON_WIDTH: f32 = SIDE_ARM_WIDTH / 3.0;

pub const LONG_BEAM_TENON_WIDTH: f32 = BEAM_STOCK_WIDTH * (3.0 / 4.0);
pub const SHORT_BEAM_LOWER_TENON_WIDTH: f32 = BEAM_STOCK_WIDTH * (2.0 / 3.0);
pub const SHORT_BEAM_UPPER_FRONT_TENON_WIDTH: f32 = BEAM_STOCK_WIDTH * (2.0 / 3.0);
pub const SHORT_BEAM_UPPER_REAR_TENON_WIDTH: f32 = BEAM_STOCK_WIDTH * (1.0 / 2.0);

pub const POST_BOARD_SIZE: [f32; 3] = [BASE_POST_HEIGHT, POST_STOCK_WIDTH, POST_STOCK_THICKNESS];

pub const LONG_BEAM_BOARD_SIZE: [f32; 3] = [
    BASE_POST_TO_POST_LENGTH + (2.0 * POST_STOCK_THICKNESS) + (2.0 * TENON_OVERRUN),
    BEAM_STOCK_WIDTH,
    BEAM_STOCK_THICKNESS,
];
