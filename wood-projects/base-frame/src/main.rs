use crate::assemblies::BaseFrame;
use parts::prelude::*;
use scad::ScadFile;
use scad_cli::BaseOpts;
use std::path::PathBuf;
use structopt::StructOpt;

mod assemblies;
mod boards;
mod config;
mod joinery;

#[derive(Debug, Clone, Eq, PartialEq, StructOpt)]
#[structopt(name = "base-frame")]
struct Opts {
    #[structopt(flatten)]
    base_opts: BaseOpts,

    /// Output directory.
    #[structopt(short = "o", long, parse(from_os_str), default_value = ".")]
    output_path: PathBuf,
}

fn main() {
    let opts = Opts::from_args();

    let mut sfile = ScadFile::new();
    sfile.set_detail(75);

    let base_frame = BaseFrame::new();

    let root = base_frame.assemble();
    sfile.add_object(root);

    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let out_file = opts.output_path.join(&filename);
    sfile
        .write_to_file(&out_file)
        .expect("Failed to write scad file");

    // TODO - do a summary output with boards, dimensions, etc
    // print all the config vars
}
