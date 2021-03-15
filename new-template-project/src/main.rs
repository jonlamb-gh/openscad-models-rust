use crate::assemblies::WallFrame;
use parts::prelude::*;
use scad::ScadFile;

mod assemblies;
mod config;

fn main() {
    let mut sfile = ScadFile::new();
    sfile.set_detail(75);

    let width = Centimeter::new(10.0);
    let thickness = Centimeter::new(5.0);
    let sep_dist = 20.0 - (5.0 / 5.0);

    let dims = BoardDimensions::new(Centimeter::new(100.0), width, thickness);
    let top_and_bottom_board = Board::with_color(dims, Color::SaddleBrown);

    let dims = BoardDimensions::new(Centimeter::new(50.0), width, thickness);
    let stud_board = Board::with_color(dims, Color::SandyBrown);

    let wall_frame = WallFrame::new(top_and_bottom_board, stud_board, sep_dist.into());

    let root = wall_frame.assemble();
    sfile.add_object(root);

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    sfile
        .write_to_file(&filename)
        .expect("Failed to write scad file");

    // The custom runner script expects the generated scad filename on stdout
    println!("{}", filename);
}
