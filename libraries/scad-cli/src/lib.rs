use structopt::StructOpt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, StructOpt)]
pub struct BaseOpts {
    /// Print summary information.
    #[structopt(short = "s", long)]
    pub summary: bool,
}
