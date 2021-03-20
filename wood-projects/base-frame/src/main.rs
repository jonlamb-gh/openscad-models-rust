use crate::assemblies::BaseFrame;
use parts::prelude::*;
use scad::ScadFile;
use scad_cli::BaseOpts;
use std::path::PathBuf;
use std::{io, io::Write};
use structopt::StructOpt;
use tabwriter::TabWriter;

mod assemblies;
mod boards;
mod config;
mod joinery;
mod project_parts;

#[derive(Debug, Clone, Eq, PartialEq, StructOpt)]
#[structopt(name = "base-frame")]
struct Opts {
    #[structopt(flatten)]
    base_opts: BaseOpts,

    /// Output directory.
    #[structopt(short = "o", long, parse(from_os_str), default_value = ".")]
    output_path: PathBuf,
}

fn main() -> io::Result<()> {
    let opts = Opts::from_args();

    let mut sfile = ScadFile::new();
    sfile.set_detail(75);

    let base_frame = BaseFrame::new(opts.base_opts.exploded_view);

    let root = base_frame.assemble();
    sfile.add_object(root);

    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let out_file = opts.output_path.join(&filename);
    sfile.write_to_file(&out_file)?;

    if opts.base_opts.summary {
        let boards = base_frame.boards();
        let mut tw = TabWriter::new(io::stdout());
        writeln!(tw, "BOARD\tDIMENSIONS\tUNITS\tBOARD FEET")?;
        for (idx, b) in boards.into_iter().enumerate() {
            writeln!(
                tw,
                "{}\t{}\t{}\t{}",
                idx + 1,
                b.dimensions(),
                b.dimensions().units(),
                b.dimensions().board_feet()
            )?;
        }
        tw.flush()?;
    }

    Ok(())
}
