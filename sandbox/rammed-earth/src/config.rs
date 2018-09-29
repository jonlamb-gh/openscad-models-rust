#![allow(dead_code)]
/// units: centimeter

pub const VISUAL_OVERRUN: f32 = 0.1;

pub const FOUNDATION_THICKNESS: f32 = 5.0;

// 9" X 9"
pub const POST_WIDTH: f32 = 22.86;
pub const POST_THICKNESS: f32 = 22.86;

// 10' X 2'
pub const OUTER_WALL_WIDTH: f32 = 304.8;
pub const OUTER_WALL_THICKNESS: f32 = 60.96;

// 4', 6', 8', 10'
pub const OUTER_WALL_L4_LENGTH: f32 = 121.92;
pub const OUTER_WALL_L6_LENGTH: f32 = 182.88;
pub const OUTER_WALL_L8_LENGTH: f32 = 243.84;
pub const OUTER_WALL_L10_LENGTH: f32 = 304.8;

// 10' X 1'
pub const INNER_WALL_WIDTH: f32 = 304.8;
pub const INNER_WALL_THICKNESS: f32 = 30.48;

// 2', 4', 5', 6', 8', 10'
pub const INNER_WALL_L2_LENGTH: f32 = 60.96;
pub const INNER_WALL_L4_LENGTH: f32 = 121.92;
pub const INNER_WALL_L5_LENGTH: f32 = 152.4;
pub const INNER_WALL_L6_LENGTH: f32 = 182.88;
pub const INNER_WALL_L8_LENGTH: f32 = 243.84;
pub const INNER_WALL_L10_LENGTH: f32 = 304.8;

//
pub const OUTER_WINDOW_FRAME_OVERRUN: f32 = 0.5;
pub const OUTER_WINDOW_FRAME_WIDTH: f32 = OUTER_WALL_THICKNESS + (2.0 * OUTER_WINDOW_FRAME_OVERRUN);
pub const OUTER_WINDOW_FRAME_THICKNESS: f32 = 2.0;

// length, height
pub const OUTER_WINDOW_L4_MAJOR: f32 = 121.92;
pub const OUTER_WINDOW_L4_MINOR: f32 = 121.92;

// 3'
pub const OUTER_WINDOW_H3_Z: f32 = 91.44;
