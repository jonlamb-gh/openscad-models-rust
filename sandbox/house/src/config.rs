#![allow(dead_code)]
/// units: centimeter

//use parts::common_functions::*;

pub const VISUAL_OVERRUN: f32 = 0.1;

// 60' X 60'
pub const FOUNDATION_SIZE: [f32; 3] = [1828.8, 1828.8, 5.0];

pub const GIRDER_BEAM_WIDTH: f32 = 25.0;
pub const GIRDER_BEAM_THICKNESS: f32 = 30.0;

// 4', 8', 16', 24', 32' plus 2*(w/2)
pub const GIRDER_BEAM_L4_LENGTH: f32 = 121.92 + GIRDER_BEAM_WIDTH;
pub const GIRDER_BEAM_L8_LENGTH: f32 = 243.84 + GIRDER_BEAM_WIDTH;
pub const GIRDER_BEAM_L16_LENGTH: f32 = 487.68 + GIRDER_BEAM_WIDTH;
pub const GIRDER_BEAM_L24_LENGTH: f32 = 731.52 + GIRDER_BEAM_WIDTH;
pub const GIRDER_BEAM_L32_LENGTH: f32 = 975.36 + GIRDER_BEAM_WIDTH;
