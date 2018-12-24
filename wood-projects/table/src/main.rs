use crate::leg::{JoineryType, Leg};
use crate::mortise_side_board::MortiseSideBoard;
use crate::table::Table;
use crate::tenon_side_board::TenonSideBoard;
use crate::top_board::{TopBoard, WidthType};
use crate::top_support_board::{SupportSide, TopSupportBoard};
use crate::wedge_board::WedgeBoard;
use dimdraw::ObjectAssembler;
use scad::{ScadFile, ScadObject};

mod axis;
mod config;
mod cutaway;
mod leg;
mod mortise_side_board;
mod quadrant;
mod table;
mod table_top;
mod tenon_side_board;
mod top_board;
mod top_support_board;
mod wedge_board;

enum Part {
    LegBoardJT0,
    LegBoardJT1,
    MajorTopBoard,
    MinorTopBoard,
    TopSupportBoardLeft,
    TopSupportBoardRight,
    TenonSideBoard,
    MortiseSideBoard,
    WedgeBoard,
    Table,
}

impl Part {
    fn assemble(&self) -> ScadObject {
        match *self {
            Part::LegBoardJT0 => Leg::new(JoineryType::JT0, None).assemble(),
            Part::LegBoardJT1 => Leg::new(JoineryType::JT1, None).assemble(),
            Part::MajorTopBoard => TopBoard::new(WidthType::Major, None).assemble(),
            Part::MinorTopBoard => TopBoard::new(WidthType::Minor, None).assemble(),
            Part::TopSupportBoardLeft => TopSupportBoard::new(SupportSide::Left, None).assemble(),
            Part::TopSupportBoardRight => TopSupportBoard::new(SupportSide::Right, None).assemble(),
            Part::TenonSideBoard => TenonSideBoard::new(None).assemble(),
            Part::MortiseSideBoard => MortiseSideBoard::new(None).assemble(),
            Part::WedgeBoard => WedgeBoard::new(None).assemble(),
            Part::Table => Table::new().assemble(),
        }
    }
}

pub fn main() {
    // Create an scad file object for storing the scad objects. This
    // allows us to set things like the detail level ($fn) for the models.
    let mut sfile = ScadFile::new();

    // Sets the $fn variable in scad which controls the detail level of things
    // like spheres. Look at the scad wiki for details
    sfile.set_detail(75);

    // Create the model
    //let part = Part::LegBoardJT0;
    //let part = Part::LegBoardJT1;
    //let part = Part::MajorTopBoard;
    //let part = Part::MinorTopBoard;
    //let part = Part::TopSupportBoardLeft;
    //let part = Part::TopSupportBoardRight;
    //let part = Part::TenonSideBoard;
    //let part = Part::MortiseSideBoard;
    //let part = Part::WedgeBoard;
    let part = Part::Table;

    // Add the model to the file
    sfile.add_object(part.assemble());

    // Save the scad code to a file
    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    // The custom runner script expects the generated scad filename on stdout
    println!("{}", filename);
}
