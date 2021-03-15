use crate::assemblies::BaseFrame;
use parts::prelude::*;
use scad::ScadFile;

mod assemblies;
mod boards;
mod config;
mod joinery;

fn main() {
    let mut sfile = ScadFile::new();
    sfile.set_detail(75);

    let base_frame = BaseFrame::new();

    let root = base_frame.assemble();
    sfile.add_object(root);

    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    sfile
        .write_to_file(&filename)
        .expect("Failed to write scad file");

    // TODO - do a summary output with boards, dimensions, etc
    // config vars
}
