//! Global configurations
//! Units: centimeter

/// TODO
pub const POST_WIDTH: f32 = 10.0;
pub const POST_THICKNESS: f32 = 10.0;

/// TODO
/// - chamfer
pub const LEG_LENGTH: f32 = 125.0;
pub const LEG_WIDTH: f32 = POST_WIDTH;
pub const LEG_THICKNESS: f32 = POST_THICKNESS;
pub const LEG_BOARD_SIZE: [f32; 3] = [LEG_LENGTH, LEG_WIDTH, LEG_THICKNESS];

/// Distance between leg center to leg center along the outside parimeter
pub const LEG_TO_LEG_DIST: f32 = 75.0;

/// TODO
/// - chamfer
pub const TOP_BOARD_LENGTH: f32 = 160.0;
pub const TOP_BOARD_WIDTH: f32 = 20.0;
pub const TOP_BOARD_THICKNESS: f32 = 4.0;
pub const TOP_BOARD_COUNT: usize = 8;
pub const TOP_BOARD_SIZE: [f32; 3] = [TOP_BOARD_LENGTH, TOP_BOARD_WIDTH, TOP_BOARD_THICKNESS];

/// TODO
pub const TOP_SUPPORT_BOARD_LENGTH: f32 = TOP_BOARD_LENGTH - TOP_BOARD_WIDTH;
pub const TOP_SUPPORT_BOARD_WIDTH: f32 = POST_WIDTH;
pub const TOP_SUPPORT_BOARD_THICKNESS: f32 = 2.5;
pub const TOP_SUPPORT_BOARD_INSET: f32 = TOP_BOARD_WIDTH / 2.0;
pub const TOP_SUPPORT_BOARD_SIZE: [f32; 3] = [
    TOP_SUPPORT_BOARD_LENGTH,
    TOP_SUPPORT_BOARD_WIDTH,
    TOP_SUPPORT_BOARD_THICKNESS,
];

/// TODO
pub const TOTAL_SIZE: [f32; 3] = [
    TOP_BOARD_LENGTH,
    TOP_BOARD_WIDTH * TOP_BOARD_COUNT as f32,
    LEG_LENGTH + TOP_SUPPORT_BOARD_THICKNESS + TOP_BOARD_THICKNESS,
];
