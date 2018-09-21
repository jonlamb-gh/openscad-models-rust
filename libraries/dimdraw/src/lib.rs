extern crate nalgebra as na;
extern crate scad;

mod arrow;
mod constants;
mod dim_line;
mod drawing_assembler;
mod leader_line;
mod line;
mod object_assembler;
mod text;
mod title_block;

pub use self::arrow::arrow;
pub use self::constants::*;
pub use self::dim_line::{dim_line, DimLocation};
pub use self::drawing_assembler::{DrawingAssembler, DrawingParams, ObjectDescriptor};
pub use self::leader_line::{leader_line, LeaderDirection, LeaderLineParams};
pub use self::line::line;
pub use self::object_assembler::ObjectAssembler;
pub use self::text::text;
//pub use self::title_block::{title_block, TitleBlockParams};

//  TODO - these need to be runtime configurable
const DOC_SCALING_FACTOR: f32 = 4.0;

const LINE_WIDTH: f32 = 0.025 * DOC_SCALING_FACTOR;

const SPACING: f32 = 0.1 * DOC_SCALING_FACTOR;

const HEIGHT: f32 = 0.01;

const FONT_SCALE: f32 = LINE_WIDTH * 0.7;
