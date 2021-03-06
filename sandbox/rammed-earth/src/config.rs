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

// 4', 6', 8', 10', 12'
pub const OUTER_WALL_L4_LENGTH: f32 = 121.92;
pub const OUTER_WALL_L6_LENGTH: f32 = 182.88;
pub const OUTER_WALL_L8_LENGTH: f32 = 243.84;
pub const OUTER_WALL_L10_LENGTH: f32 = 304.8;
pub const OUTER_WALL_L12_LENGTH: f32 = 365.76;

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

pub const OUTER_WINDOW_FRAME_OVERRUN: f32 = 0.5;
pub const OUTER_WINDOW_FRAME_WIDTH: f32 = OUTER_WALL_THICKNESS + (2.0 * OUTER_WINDOW_FRAME_OVERRUN);
pub const OUTER_WINDOW_FRAME_THICKNESS: f32 = 2.0;

pub const OUTER_WINDOW_SQ4_MAJOR: f32 = 121.92;
pub const OUTER_WINDOW_SQ4_MINOR: f32 = 121.92;

pub const OUTER_WINDOW_SQ2_MAJOR: f32 = 60.96;
pub const OUTER_WINDOW_SQ2_MINOR: f32 = 60.96;

pub const OUTER_WINDOW_8X2_MAJOR: f32 = 243.84;
pub const OUTER_WINDOW_8X2_MINOR: f32 = 60.96;

pub const OUTER_WINDOW_6X2_MAJOR: f32 = 182.88;
pub const OUTER_WINDOW_6X2_MINOR: f32 = 60.96;

pub const OUTER_WINDOW_7X5_MAJOR: f32 = 213.36;
pub const OUTER_WINDOW_7X5_MINOR: f32 = 152.4;

// 3', 5', 6.3'
pub const OUTER_WINDOW_H3_Z: f32 = 91.44;
pub const OUTER_WINDOW_H5_Z: f32 = 152.4;
pub const OUTER_WINDOW_H6P3_Z: f32 = 192.024;

pub const INNER_WINDOW_FRAME_OVERRUN: f32 = 0.5;
pub const INNER_WINDOW_FRAME_WIDTH: f32 = INNER_WALL_THICKNESS + (2.0 * INNER_WINDOW_FRAME_OVERRUN);
pub const INNER_WINDOW_FRAME_THICKNESS: f32 = 2.0;

pub const INNER_WINDOW_1X6_MAJOR: f32 = 30.48;
pub const INNER_WINDOW_1X6_MINOR: f32 = 182.88;

pub const INNER_WINDOW_H2_Z: f32 = 60.96;

pub const DOOR_FRAME_OVERRUN: f32 = 0.5;
pub const DOOR_FRAME_WIDTH: f32 = OUTER_WALL_THICKNESS + (2.0 * DOOR_FRAME_OVERRUN);
pub const DOOR_FRAME_THICKNESS: f32 = 2.0;

pub const DOOR_THICKNESS: f32 = 10.0;

pub const SINGLE_DOOR_MAJOR: f32 = 106.68;
pub const SINGLE_DOOR_MINOR: f32 = 213.36;

pub const DOUBLE_DOOR_MAJOR: f32 = 212.0;
pub const DOUBLE_DOOR_MINOR: f32 = 213.36;

pub const ROOF_SLOPE_ANGLE: f32 = 6.0;

// 2'
pub const RAFTER_OVERHANG: f32 = 60.96;

// TODO - use standard, go from outside in, middle gap consumes remainder
pub const RAFTER_SEP_DISTANCE: f32 = 174.4;

// 8" X 6"
pub const RAFTER_WIDTH: f32 = 20.32;
pub const RAFTER_THICKNESS: f32 = 15.24;

// 6" X 6"
pub const RAFTER_BEAM_WIDTH: f32 = 15.24;
pub const RAFTER_BEAM_THICKNESS: f32 = 15.24;

// 36'
pub const RAFTER_SPAN_LENGTH: f32 = 1097.28;
