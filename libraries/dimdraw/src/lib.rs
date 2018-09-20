extern crate nalgebra as na;
extern crate scad;

// TODO - clean this up
#[path = "arrow.rs"]
mod arrow;
#[path = "line.rs"]
mod line;
#[path = "text.rs"]
mod text;

pub use self::arrow::arrow;
pub use self::line::line;
pub use self::text::text;

const LINE_WIDTH: f32 = 0.025;

// const SPACE = 0.1

const HEIGHT: f32 = 0.01;

const FONT_SCALE: f32 = LINE_WIDTH * 0.7;
