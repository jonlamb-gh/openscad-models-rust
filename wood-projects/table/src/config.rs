//! Global configurations
//! Units: centimeter

pub const VISUAL_OVERRUN: f32 = 0.1;

// TODO
/// Distance between leg center to leg center along the outside parimeter
pub const MAJOR_LEG_TO_LEG_DIST: f32 = 78.74;
pub const MINOR_LEG_TO_LEG_DIST: f32 = 70.0;

pub const TOP_SUPPORT_CUTOUT_DEPTH: f32 = 1.0;

pub const TENON_OVERRUN: f32 = 5.0;
pub const MORTISE_BOARD_TENON_OVERRUN: f32 = 1.0;

pub const LEG_CHAMFER_RATIO: f32 = 1.0 / 16.0;

pub const TOP_CHAMFER_RATIO: f32 = 1.0 / 8.0;

pub const SIDE_TENON_RATIO: f32 = 5.0 / 8.0;

pub const WEDGE_LENGTH: f32 = 6.0;
pub const WEDGE_WIDTH: f32 = 1.9;
pub const WEDGE_THICKNESS: f32 = 1.5;
pub const WEDGE_BOARD_SIZE: [f32; 3] = [WEDGE_LENGTH, WEDGE_WIDTH, WEDGE_THICKNESS];

/// TODO
/// 3" == 7.62 cm
/// 1.5" == 3.81 cm
pub const POST_WIDTH: f32 = 7.62;
pub const POST_THICKNESS: f32 = 7.62;

/// TODO
/// - chamfer
/// - 36" == 91.44 cm
/// - ~30.7" == 78 cm
/// - 33.5
pub const LEG_LENGTH: f32 = 78.0;
pub const LEG_WIDTH: f32 = POST_WIDTH;
pub const LEG_THICKNESS: f32 = POST_THICKNESS;
pub const LEG_CHAMFER_LENGTH: f32 = SIDE_SUPPORT_BOARD_HEIGHT - SIDE_SUPPORT_BOARD_WIDTH;
pub const LEG_CHAMFER_DEPTH: f32 = LEG_CHAMFER_LENGTH * LEG_CHAMFER_RATIO;
pub const LEG_BOARD_SIZE: [f32; 3] = [LEG_LENGTH, LEG_WIDTH, LEG_THICKNESS];

/// TODO
/// - chamfer
/// - rough length 46" == 116.84 cm
/// - rough major width 9.84252" == 25 cm
/// - rough minor width 5.23622" == 13.3 cm
/// - rough thickness 1.29921" == 3.3 cm
pub const TOP_BOARD_LENGTH: f32 = 115.0;
pub const TOP_BOARD_THICKNESS: f32 = 3.3;
pub const TOP_BOARD_MAJOR_WIDTH: f32 = 25.0;
pub const TOP_BOARD_MINOR_WIDTH: f32 = 13.0;
pub const TOP_BOARD_CHAMFER_LENGTH: f32 = 15.0;
pub const TOP_BOARD_CHAMFER_DEPTH: f32 = TOP_BOARD_CHAMFER_LENGTH * TOP_CHAMFER_RATIO;
/// 3 major boards, 3 minor boards
pub const TOP_BOARD_MAJOR_COUNT: usize = 3;
pub const TOP_BOARD_MINOR_COUNT: usize = 2;
pub const TOP_BOARD_COUNT: usize = TOP_BOARD_MAJOR_COUNT + TOP_BOARD_MINOR_COUNT;
pub const TOP_TOTAL_WIDTH: f32 = (TOP_BOARD_MAJOR_COUNT as f32 * TOP_BOARD_MAJOR_WIDTH)
    + (TOP_BOARD_MINOR_COUNT as f32 * TOP_BOARD_MINOR_WIDTH);
pub const TOP_BOARD_MAJOR_SIZE: [f32; 3] =
    [TOP_BOARD_LENGTH, TOP_BOARD_MAJOR_WIDTH, TOP_BOARD_THICKNESS];
pub const TOP_BOARD_MINOR_SIZE: [f32; 3] =
    [TOP_BOARD_LENGTH, TOP_BOARD_MINOR_WIDTH, TOP_BOARD_THICKNESS];

/// TODO
pub const TOP_SUPPORT_BOARD_LENGTH: f32 = TOP_TOTAL_WIDTH - (2.0 * TOP_SUPPORT_BOARD_INSET);
pub const TOP_SUPPORT_BOARD_WIDTH: f32 = 5.0;
pub const TOP_SUPPORT_BOARD_THICKNESS: f32 = 3.3;
pub const TOP_SUPPORT_BOARD_INSET: f32 = 9.0;
pub const TOP_SUPPORT_BOARD_SIZE: [f32; 3] = [
    TOP_SUPPORT_BOARD_LENGTH,
    TOP_SUPPORT_BOARD_WIDTH,
    TOP_SUPPORT_BOARD_THICKNESS,
];

/// TODO
pub const TENON_SIDE_SUPPORT_BOARD_LENGTH: f32 =
    MAJOR_LEG_TO_LEG_DIST + POST_THICKNESS + (2.0 * TENON_OVERRUN);
pub const MORTISE_SIDE_SUPPORT_BOARD_LENGTH: f32 =
    MINOR_LEG_TO_LEG_DIST + POST_THICKNESS + (2.0 * MORTISE_BOARD_TENON_OVERRUN);
pub const SIDE_SUPPORT_BOARD_WIDTH: f32 = 10.0;
pub const SIDE_SUPPORT_BOARD_THICKNESS: f32 = 1.9;
pub const SIDE_SUPPORT_BOARD_HEIGHT: f32 = LEG_LENGTH - (TOP_SUPPORT_BOARD_THICKNESS / 2.0);
pub const SIDE_SUPPORT_TENON_LENGTH: f32 = POST_THICKNESS + TENON_OVERRUN;
pub const SIDE_SUPPORT_TENON_WIDTH: f32 = SIDE_SUPPORT_BOARD_WIDTH * SIDE_TENON_RATIO;
pub const SIDE_SUPPORT_MORTISE_LENGTH: f32 = SIDE_SUPPORT_BOARD_THICKNESS;
pub const SIDE_SUPPORT_MORTISE_WIDTH: f32 = SIDE_SUPPORT_TENON_WIDTH;
pub const TENON_SIDE_SUPPORT_BOARD_SIZE: [f32; 3] = [
    TENON_SIDE_SUPPORT_BOARD_LENGTH,
    SIDE_SUPPORT_BOARD_WIDTH,
    SIDE_SUPPORT_BOARD_THICKNESS,
];
pub const MORTISE_SIDE_SUPPORT_BOARD_SIZE: [f32; 3] = [
    MORTISE_SIDE_SUPPORT_BOARD_LENGTH,
    SIDE_SUPPORT_BOARD_WIDTH,
    SIDE_SUPPORT_BOARD_THICKNESS,
];

/// TODO
pub const TOTAL_SIZE: [f32; 3] = [
    TOP_BOARD_LENGTH,
    TOP_TOTAL_WIDTH,
    LEG_LENGTH + TOP_SUPPORT_BOARD_THICKNESS + TOP_BOARD_THICKNESS,
];
