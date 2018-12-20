use crate::leg::Leg;
use crate::mortise_side_board::MortiseSideBoard;
use crate::table::Table;
use crate::tenon_side_board::TenonSideBoard;
use crate::top_board::{TopBoard, WidthType};
use crate::top_support_board::TopSupportBoard;
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
    LegBoard,
    MajorTopBoard,
    MinorTopBoard,
    TopSupportBoard,
    TenonSideBoard,
    MortiseSideBoard,
    WedgeBoard,
    Table,
}

impl Part {
    fn assemble(&self) -> ScadObject {
        match *self {
            Part::LegBoard => Leg::new(None).assemble(),
            Part::MajorTopBoard => TopBoard::new(WidthType::Major, None).assemble(),
            Part::MinorTopBoard => TopBoard::new(WidthType::Minor, None).assemble(),
            Part::TopSupportBoard => TopSupportBoard::new(None).assemble(),
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
    sfile.set_detail(50);

    // Create the model
    //let part = Part::LegBoard;
    //let part = Part::MajorTopBoard;
    //let part = Part::MinorTopBoard;
    //let part = Part::TopSupportBoard;
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
