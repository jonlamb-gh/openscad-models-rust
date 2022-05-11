use crate::assemblies::Shelves;
use parts::prelude::*;
use scad::ScadFile;
use scad_cli::BaseOpts;
use std::collections::HashMap;
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

    let shelves = Shelves::new(opts.base_opts.exploded_view);
    let root = shelves.assemble();
    sfile.add_object(root);

    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let out_file = opts.output_path.join(&filename);
    sfile.write_to_file(&out_file)?;

    // TODO - double check the in-to-cm hack in parts lib
    if opts.base_opts.summary {
        let boards = shelves.boards();
        let mut board_counts: HashMap<Board<Inch>, usize> = HashMap::new();
        for b in boards.into_iter() {
            let count = board_counts.entry(*b).or_insert(0);
            *count += 1;
        }
        let mut tw = TabWriter::new(io::stdout());
        writeln!(tw, "NUM BOARDS\tDIMENSIONS\tUNITS\tBOARD FEET")?;
        for (board, count) in board_counts.into_iter() {
            let total_board_feet = board.dimensions().board_feet() * (count as f32);
            writeln!(
                tw,
                "{}\t{}\t{}\t{}",
                count,
                board.dimensions(),
                board.dimensions().units(),
                total_board_feet
            )?;
        }
        tw.flush()?;
    }

    Ok(())
}
