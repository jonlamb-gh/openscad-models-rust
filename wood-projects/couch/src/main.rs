#![feature(const_fn, const_panic)]

// TODO - use dimensioned crate for units?

//#[macro_use]
extern crate dimdraw;
extern crate scad;

use scad::*;

mod board;
mod board_dimensions;
mod common_functions;
mod config;
mod couch;
mod cutaway;
mod long_beam;
mod lower_short_beam;
mod post;
mod post_board;
mod support_plank;
mod upper_short_beam;

use couch::Couch;
use dimdraw::{DrawingAssembler, ObjectAssembler};

fn main() {
    // Create the part
    let part = Couch::new();

    // Create an scad file object for storing the scad objects. This
    // allows us to set things like the detail level ($fn) for the models.
    let mut sfile = ScadFile::new();

    // Sets the $fn variable in scad which controls the detail level of things
    // like spheres. Look at the scad wiki for details
    sfile.set_detail(100);

    // Add the model to the file
    sfile.add_object(part.assemble());

    // Add model drawing to the file
    //sfile.add_object(part.assemble_drawing());

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    // The custom runner script expects the generated scad filename on stdout
    println!("{}", filename);
}
