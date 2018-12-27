use crate::leg::{JoineryType, Leg};
use crate::mortise_side_board::MortiseSideBoard;
use crate::table::Table;
use crate::tenon_side_board::TenonSideBoard;
use crate::top_board::{TopBoard, WidthType};
use crate::top_support_board::{SupportSide, TopSupportBoard};
use crate::wedge_board::WedgeBoard;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::ScadObject;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Part {
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

// TODO
/*
impl Part {
    fn abs_pos
}
*/

// TODO - clean this up
impl ObjectAssembler for Part {
    fn describe(&self) -> ObjectDescriptor {
        match *self {
            Part::LegBoardJT0 => Leg::new(JoineryType::JT0, None).describe(),
            Part::LegBoardJT1 => Leg::new(JoineryType::JT1, None).describe(),
            Part::MajorTopBoard => TopBoard::new(WidthType::Major, None).describe(),
            Part::MinorTopBoard => TopBoard::new(WidthType::Minor, None).describe(),
            Part::TopSupportBoardLeft => TopSupportBoard::new(SupportSide::Left, None).describe(),
            Part::TopSupportBoardRight => TopSupportBoard::new(SupportSide::Right, None).describe(),
            Part::TenonSideBoard => TenonSideBoard::new(None).describe(),
            Part::MortiseSideBoard => MortiseSideBoard::new(None).describe(),
            Part::WedgeBoard => WedgeBoard::new(None).describe(),
            Part::Table => Table::new().describe(),
        }
    }

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
