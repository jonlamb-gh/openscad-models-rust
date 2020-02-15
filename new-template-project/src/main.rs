use crate::assemblies::WallFrame;
use parts::utils::{BoardDimensions, Color};
use parts::Board;
use scad::ScadFile;
use scad_assembler::ScadAssembler;

mod assemblies;
mod config;

pub fn main() {
    // Create an scad file object for storing the scad objects. This
    // allows us to set things like the detail level ($fn) for the models.
    let mut sfile = ScadFile::new();

    // Sets the $fn variable in scad which controls the detail level of things
    // like spheres. Look at the scad wiki for details
    sfile.set_detail(75);

    // units: cm
    let dims = BoardDimensions::new(100.0, 10.0, 5.0);
    let top_and_bottom_board = Board::new(dims, Some(Color::PapayaWhip));

    let dims = BoardDimensions::new(50.0, 10.0, 5.0);
    let stud_board = Board::new(dims, Some(Color::SandyBrown));

    let sep_dist = 20.0 - (5.0 / 5.0);
    println!("sep {}", sep_dist);
    let wall_frame = WallFrame::new(top_and_bottom_board, stud_board, sep_dist);

    sfile.add_object(wall_frame.assemble());

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    // The custom runner script expects the generated scad filename on stdout
    println!("{}", filename);
}
