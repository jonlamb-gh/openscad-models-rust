use crate::axis::Axis;
use crate::config::*;
use crate::leg::{JoineryType, Leg};
use crate::mortise_side_board::{MortiseSide, MortiseSideBoard};
use crate::quadrant::Quadrant;
use crate::table_top::TableTop;
use crate::tenon_side_board::{TenonSide, TenonSideBoard};
use crate::wedge_board::WedgeBoard;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

qstruct!(Table() {
    leg_type0: Leg = Leg::new(JoineryType::JT0, Some("Peru")),
    leg_type1: Leg = Leg::new(JoineryType::JT1, Some("Peru")),
    top: TableTop = TableTop::new(Some("SaddleBrown"), Some("SandyBrown")),
    tenon_side_board: TenonSideBoard = TenonSideBoard::new(Some("BurlyWood")),
    mortise_side_board: MortiseSideBoard = MortiseSideBoard::new(Some("BurlyWood")),
    wedge: WedgeBoard = WedgeBoard::new(Some("SaddleBrown")),
});

impl Table {
    pub fn assemble_legs(&self) -> ScadObject {
        let q0 = Leg::abs_pos(Quadrant::Q0, false);
        let q1 = Leg::abs_pos(Quadrant::Q1, false);
        let q2 = Leg::abs_pos(Quadrant::Q2, false);
        let q3 = Leg::abs_pos(Quadrant::Q3, false);

        scad!(Union;{
            scad!(Translate(q0);{
                self.leg_type0.assemble_aligned(Quadrant::Q0),
            }),
            scad!(Translate(q1);{
                self.leg_type1.assemble_aligned(Quadrant::Q1),
            }),
            scad!(Translate(q2);{
                self.leg_type0.assemble_aligned(Quadrant::Q2),
            }),
            scad!(Translate(q3);{
                self.leg_type1.assemble_aligned(Quadrant::Q3),
            }),
        })
    }

    pub fn assemble_top(&self) -> ScadObject {
        let z = LEG_LENGTH - (TOP_SUPPORT_BOARD_THICKNESS / 2.0) - TOP_SUPPORT_CUTOUT_DEPTH;
        scad!(Translate(vec3(0.0, 0.0, z));{
            self.top.assemble()
        })
    }

    pub fn assemble_sides(&self) -> ScadObject {
        let t_front = TenonSideBoard::abs_pos(TenonSide::Front);
        let t_back = TenonSideBoard::abs_pos(TenonSide::Back);
        let m_left = MortiseSideBoard::abs_pos(MortiseSide::Left);
        let m_right = MortiseSideBoard::abs_pos(MortiseSide::Right);

        scad!(Union;{
            scad!(Translate(t_front);{
                self.tenon_side_board.assemble_aligned(Axis::X),
            }),
            scad!(Translate(t_back);{
                self.tenon_side_board.assemble_aligned(Axis::X),
            }),
            scad!(Translate(m_left);{
                self.mortise_side_board.assemble_aligned(Axis::Y),
            }),
            scad!(Translate(m_right);{
                self.mortise_side_board.assemble_aligned(Axis::Y),
            }),
        })
    }

    pub fn assemble_wedges(&self) -> ScadObject {
        let q0 = WedgeBoard::abs_pos(Quadrant::Q0);
        let q1 = WedgeBoard::abs_pos(Quadrant::Q1);
        let q2 = WedgeBoard::abs_pos(Quadrant::Q2);
        let q3 = WedgeBoard::abs_pos(Quadrant::Q3);

        scad!(Union;{
            scad!(Translate(q0);{
                self.wedge.assemble_aligned(),
            }),
            scad!(Translate(q1);{
                self.wedge.assemble_aligned(),
            }),
            scad!(Translate(q2);{
                self.wedge.assemble_aligned(),
            }),
            scad!(Translate(q3);{
                self.wedge.assemble_aligned(),
            }),
        })
    }
}

impl ObjectAssembler for Table {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: TOTAL_SIZE[0],
            width: TOTAL_SIZE[1],
            thickness: TOTAL_SIZE[2],
        }
    }

    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.assemble_legs(),
            self.assemble_top(),
            self.assemble_sides(),
            self.assemble_wedges(),
        })
    }
}
