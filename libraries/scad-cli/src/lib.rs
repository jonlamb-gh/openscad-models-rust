use structopt::StructOpt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, StructOpt)]
pub struct BaseOpts {
    /// Print summary information.
    #[structopt(short = "s", long)]
    pub summary: bool,

    /// Render an exploded view.
    #[structopt(short = "e", long)]
    pub exploded_view: bool,

    /// Omit colors when rendering.
    #[structopt(long)]
    pub no_color: bool,
}
