//#[macro_use]
extern crate scad;

use scad::*;

pub fn main() {
    // Create an scad file object for storing the scad objects. This
    // allows us to set things like the detail level ($fn) for the models.
    let mut sfile = ScadFile::new();

    // Sets the $fn variable in scad which controls the detail level of things
    // like spheres. Look at the scad wiki for details
    sfile.set_detail(50);

    // Create an scad object
    let mut cube = scad!(Translate(vec3(2.0, 2.0, 3.0));
            {
                scad!(Cube(vec3(2.0,1.0,4.0)))
            });

    // Create a cylinder with a height of 10 and a diameter of 3 mm
    let cylinder = scad!(Cylinder(10., Diameter(3.)));

    // Add the cylinder to the cubes translation.
    cube.add_child(cylinder);

    // Add the cube object to the file
    sfile.add_object(cube.clone());

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    sfile.write_to_file(filename.to_string());

    // Our customer runner script expects the generated scad filename on stdout
    println!("{}", filename);
}
