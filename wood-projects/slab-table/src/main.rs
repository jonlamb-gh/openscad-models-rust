extern crate dimdraw;
extern crate nalgebra as na;
extern crate parts;
extern crate scad;

mod config;
mod post;
mod post_drawing;
mod slab;
mod slab_table;

use dimdraw::DrawingAssembler;
use dimdraw::ObjectAssembler;
use scad::*;

use slab_table::SlabTable;

fn main() {
    // Create the part
    let part = SlabTable::new();

    // Create an scad file object for storing the scad objects. This
    // allows us to set things like the detail level ($fn) for the models.
    let mut sfile = ScadFile::new();

    // Sets the $fn variable in scad which controls the detail level of things
    // like spheres. Look at the scad wiki for details
    sfile.set_detail(100);

    // Add the model to the file
    if cfg!(feature = "assembled-drawing") {
        sfile.add_object(part.assemble_drawing());
    } else if cfg!(feature = "post-drawing") {
        sfile.add_object(part.post.assemble_drawing());
    } else if cfg!(feature = "slab-drawing") {
        sfile.add_object(part.slab.assemble_drawing());
    } else if cfg!(feature = "pieces") {
        sfile.add_object(scad!(Union;{
            part.post.assemble_aligned(),
            scad!(Translate(vec3(30.0, 20.0, 0.0));{
                part.slab.assemble(),
            }),
            scad!(Translate(vec3(30.0, -20.0, 0.0));{
                part.post.assemble(),
            }),
        }));
    } else {
        sfile.add_object(part.assemble());
    }

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    // The custom runner script expects the generated scad filename on stdout
    // argv[0] = path/to/filename.scad
    // argv[1+] = arguments given to OpenSCAD
    println!("{}", filename);
}
