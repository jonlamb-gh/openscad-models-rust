extern crate nalgebra as na;
extern crate scad;

// TODO - clean this up
#[path = "arrow.rs"]
mod arrow;
#[path = "dim_line.rs"]
mod dim_line;
#[path = "leader_line.rs"]
mod leader_line;
#[path = "line.rs"]
mod line;
#[path = "text.rs"]
mod text;
#[path = "title_block.rs"]
mod title_block;

pub use self::arrow::arrow;
pub use self::dim_line::{dim_line, DimLocation};
pub use self::leader_line::{leader_line, LeaderDirection, LeaderLineParams};
pub use self::line::line;
pub use self::text::text;
//pub use self::title_block::{title_block, TitleBlockParams};

const LINE_WIDTH: f32 = 0.025;

// const SPACE = 0.1

const HEIGHT: f32 = 0.01;

const FONT_SCALE: f32 = LINE_WIDTH * 0.7;
