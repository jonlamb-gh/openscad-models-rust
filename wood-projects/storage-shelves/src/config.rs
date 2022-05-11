use parts::prelude::*;

pub const PARTICLE_BOARD_THICKNESS: Inch = Inch::new(0.5);

// 2x4
pub const BOARD_WIDTH: Inch = Inch::new(3.5);
pub const BOARD_THICKNESS: Inch = Inch::new(1.5);

pub const LONG_BOARD_LENGTH: Inch = Inch::new(96.0);
pub const MED_BOARD_LENGTH: Inch = Inch::new(48.0);
pub const SHORT_BOARD_LENGTH: Inch = Inch::new(27.0);

pub const LONG_BOARD_COLOR: Color = Color::SandyBrown;
pub const MED_BOARD_COLOR: Color = Color::SandyBrown;
pub const SHORT_BOARD_COLOR: Color = Color::SaddleBrown;

/// Full shelves have supports every 31.5 inches, on center
pub const FULL_SHELF_SUPPORT_BOARD_OFFSET: Inch = Inch::new(31.5);
pub const NUM_SHORT_BOARDS_PER_FULL_SHELF: usize = 4;

/// Half shelves have supports every 23.25 inches, on center
pub const HALF_SHELF_SUPPORT_BOARD_OFFSET: Inch = Inch::new(23.25);
pub const NUM_SHORT_BOARDS_PER_HALF_SHELF: usize = 3;

pub const HORIZONTAL_SECTION_LENGTHS: [Inch; 3] =
    [MED_BOARD_LENGTH, LONG_BOARD_LENGTH, MED_BOARD_LENGTH];

/// Heights (vertical space) between each shelf, starting
/// at ground level
pub const VERTICAL_SHELF_HEIGHTS: [Inch; 5] = [
    Inch::new(12.0),
    // 1'8" from top to next top
    Inch::new(20.0 - BOARD_WIDTH.as_f32()), // TODO - account for plywood thickness
    Inch::new(20.0 - BOARD_WIDTH.as_f32()), // TODO - account for plywood thickness
    Inch::new(20.0 - BOARD_WIDTH.as_f32()), // TODO - account for plywood thickness
    Inch::new(20.0 - BOARD_WIDTH.as_f32()), // TODO - account for plywood thickness
];

/// One on each end, another MED_BOARD_LENGTH in, on center
pub const NUM_VERTICAL_SUPPORTS: usize = 4;

pub const SCREW_HOLE_DIAMETER: Inch = Inch::new(0.25);

pub fn vertical_shelf_accum_heights() -> Vec<Inch> {
    let mut vo: Vec<Inch> = Vec::new();
    for (idx, _h) in VERTICAL_SHELF_HEIGHTS.into_iter().enumerate() {
        let o = (Inch::new(idx as _) * BOARD_WIDTH)
            + VERTICAL_SHELF_HEIGHTS[..idx + 1]
                .iter()
                .fold(Inch::new(0.0), |mut sum, val| {
                    sum = sum + *val;
                    sum
                });
        vo.push(o);
    }
    vo
}
