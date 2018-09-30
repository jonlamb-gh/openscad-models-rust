extern crate dimdraw;
extern crate nalgebra as na;
extern crate parts;
extern crate scad;

mod config;
mod doors;
mod foundation;
mod house;
mod inner_walls;
mod outer_walls;

use dimdraw::ObjectAssembler;
use scad::*;

use house::House;

fn main() {
    // Create the part
    let part = House::new();

    // Create an scad file object for storing the scad objects. This
    // allows us to set things like the detail level ($fn) for the models.
    let mut sfile = ScadFile::new();

    // Sets the $fn variable in scad which controls the detail level of things
    // like spheres. Look at the scad wiki for details
    sfile.set_detail(100);

    // Add the model to the file
    //sfile.add_object(part.assemble_center_xy());
    sfile.add_object(part.assemble());

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    // The custom runner script expects the generated scad filename on stdout
    println!("{}", filename);
}
