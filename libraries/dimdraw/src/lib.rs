extern crate nalgebra as na;
extern crate scad;

// TODO - clean this up once restored back to a proper lib
#[path = "arrow.rs"]
mod arrow;
#[path = "constants.rs"]
mod constants;
#[path = "dim_line.rs"]
mod dim_line;
#[path = "drawing_assembler.rs"]
mod drawing_assembler;
#[path = "leader_line.rs"]
mod leader_line;
#[path = "line.rs"]
mod line;
#[path = "object_assembler.rs"]
mod object_assembler;
#[path = "text.rs"]
mod text;
#[path = "title_block.rs"]
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

const DOC_SCALING_FACTOR: f32 = 4.0;

const LINE_WIDTH: f32 = 0.025 * DOC_SCALING_FACTOR;

const SPACING: f32 = 0.1 * DOC_SCALING_FACTOR;

const HEIGHT: f32 = 0.01;

const FONT_SCALE: f32 = LINE_WIDTH * 0.7;
