use crate::assemblies::BaseFrame;
use scad::ScadFile;
use scad_assembler::ScadAssembler;

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

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    sfile
        .write_to_file(&filename)
        .expect("Failed to write scad file");

    // The custom runner script expects the generated scad filename on stdout
    println!("{}", filename);
}
