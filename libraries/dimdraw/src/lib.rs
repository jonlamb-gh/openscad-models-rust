extern crate nalgebra as na;
extern crate scad;

// TODO - clean this up
#[path = "arrow.rs"]
mod arrow;
#[path = "line.rs"]
mod line;

pub use self::arrow::arrow;
pub use self::line::line;

const WIDTH: f32 = 0.025;
const HEIGHT: f32 = 0.01;
