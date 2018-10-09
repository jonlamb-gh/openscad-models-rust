extern crate nalgebra as na;
extern crate scad;

mod arrow;
pub mod constants;
mod cross;
mod dim_line;
mod drawing;
mod drawing_assembler;
mod leader_line;
mod line;
mod object_assembler;
mod text;

pub use self::dim_line::DimLocation;
pub use self::drawing::Drawing;
pub use self::drawing_assembler::{DrawingAssembler, DrawingParams, Viewport};
pub use self::leader_line::{LeaderDirection, LeaderLineParams};
pub use self::object_assembler::{some_color, ObjectAssembler, ObjectDescriptor};

pub const DOC_THICKNESS: f32 = 0.01;
