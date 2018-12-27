use crate::part::Part;
use dimdraw::ObjectAssembler;
use scad::ScadFile;

mod axis;
mod config;
mod cutaway;
mod leg;
mod mortise_side_board;
mod part;
mod quadrant;
mod table;
mod table_top;
mod tenon_side_board;
mod top_board;
mod top_support_board;
mod wedge_board;

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
