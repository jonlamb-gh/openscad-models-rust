extern crate scad;

mod lib;

use lib::*;
use scad::*;

fn main() {
    // Create an scad file object for storing the scad objects. This
    // allows us to set things like the detail level ($fn) for the models.
    let mut sfile = ScadFile::new();

    // Sets the $fn variable in scad which controls the detail level of things
    // like spheres. Look at the scad wiki for details
    sfile.set_detail(50);

    // Create an scad object
    let mut object = scad!(Color(vec3(0.0, 0.0, 0.0)));

    object.add_child(line(15.0, true, true));

    // Add the cube object to the file
    sfile.add_object(object);

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    // The custom runner script expects the generated scad filename on stdout
    println!("{}", filename);
}
