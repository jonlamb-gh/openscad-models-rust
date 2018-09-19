#![feature(const_fn, const_panic)]

// TODO - use dimensioned crate for units?

//#[macro_use]
extern crate scad;

use scad::*;

mod board;
mod board_dimensions;
mod common_functions;
mod config;
mod couch;
mod cutaway;
mod object_assembler;
mod post;
mod post_board;

use couch::Couch;
use object_assembler::ObjectAssembler;

fn main() {
    // Create an scad file object for storing the scad objects. This
    // allows us to set things like the detail level ($fn) for the models.
    let mut sfile = ScadFile::new();

    // Sets the $fn variable in scad which controls the detail level of things
    // like spheres. Look at the scad wiki for details
    sfile.set_detail(50);

    // Add the model to the file
    let couch = Couch::new();
    sfile.add_object(couch.assemble());

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    // The custom runner script expects the generated scad filename on stdout
    println!("{}", filename);
}
