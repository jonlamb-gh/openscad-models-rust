#![allow(dead_code)]
/// units: centimeter

pub const VISUAL_OVERRUN: f32 = 0.1;

pub const TENON_OVERRUN: f32 = 0.5;

/// Depth of the post tenon cutout along the width/y-axis
pub const POST_TENON_MAJOR_DEPTH: f32 = 5.0;

/// Depth of the post tenon cutout along the thickness/z-axis
pub const POST_TENON_MINOR_DEPTH: f32 = 7.0;

pub const SLAB_LENGTH: f32 = 238.0;
pub const SLAB_WIDTH: f32 = 73.0;
pub const SLAB_THICKNESS: f32 = 5.5;

pub const POST_LENGTH: f32 = 48.26;
pub const POST_WIDTH: f32 = 25.4;
pub const POST_THICKNESS: f32 = 20.32;

/// Distance from edge of slab to outer face of post
///
/// Distance to center of post is POST_TO_EDGE_DIST + (POST_THICKNESS/2).
pub const POST_TO_EDGE_DIST: f32 = 40.64;
