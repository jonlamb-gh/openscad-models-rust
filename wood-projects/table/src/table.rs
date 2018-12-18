use crate::config::*;
use crate::leg::{Leg, Quadrant};
use crate::mortise_side_board::MortiseSideBoard;
use crate::side_board::Axis;
use crate::table_top::TableTop;
use crate::tenon_side_board::TenonSideBoard;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

qstruct!(Table() {
    leg: Leg = Leg::new(Some("Peru")),
    top: TableTop = TableTop::new(Some("SaddleBrown"), Some("SandyBrown")),
    tenon_side_board: TenonSideBoard = TenonSideBoard::new(Some("BurlyWood")),
    mortise_side_board: MortiseSideBoard = MortiseSideBoard::new(Some("BurlyWood")),
});

impl Table {
    pub fn assemble_legs(&self) -> ScadObject {
        let cx = self.top.describe().length / 2.0;
        let cy = self.top.describe().width / 2.0;

        // Center the legs relative to the top
        let x0 = cx - (LEG_TO_LEG_DIST / 2.0) - (self.leg.describe().thickness / 2.0);
        let x1 = cx + (LEG_TO_LEG_DIST / 2.0) - (self.leg.describe().thickness / 2.0);
        let y0 = cy - (LEG_TO_LEG_DIST / 2.0) - (self.leg.describe().width / 2.0);
        let y1 = cy + (LEG_TO_LEG_DIST / 2.0) - (self.leg.describe().width / 2.0);

        scad!(Union;{
            scad!(Translate(vec3(x0, y0, 0.0));{
                self.leg.assemble_aligned(Quadrant::Q0),
            }),
            scad!(Translate(vec3(x0, y1, 0.0));{
                self.leg.assemble_aligned(Quadrant::Q1),
            }),
            scad!(Translate(vec3(x1, y1, 0.0));{
                self.leg.assemble_aligned(Quadrant::Q2),
            }),
            scad!(Translate(vec3(x1, y0, 0.0));{
                self.leg.assemble_aligned(Quadrant::Q3),
            }),
        })
    }

    pub fn assemble_top(&self) -> ScadObject {
        let z = self.leg.describe().length
            - (TOP_SUPPORT_BOARD_THICKNESS / 2.0)
            - TOP_SUPPORT_CUTOUT_DEPTH;
        scad!(Translate(vec3(0.0, 0.0, z));{
            self.top.assemble()
        })
    }

    pub fn assemble_sides(&self) -> ScadObject {
        let cx = self.top.describe().length / 2.0;
        let cy = self.top.describe().width / 2.0;

        let z = SIDE_SUPPORT_BOARD_HEIGHT - SIDE_SUPPORT_BOARD_WIDTH;
        let x0 =
            cx - (LEG_TO_LEG_DIST / 2.0) - (self.leg.describe().thickness / 2.0) - TENON_OVERRUN;
        let x1 =
            cx - (LEG_TO_LEG_DIST / 2.0) - (self.mortise_side_board.describe().thickness / 2.0);
        let x2 =
            cx + (LEG_TO_LEG_DIST / 2.0) - (self.mortise_side_board.describe().thickness / 2.0);
        let y0 = cy - (LEG_TO_LEG_DIST / 2.0) - (self.tenon_side_board.describe().thickness / 2.0);
        let y1 = cy + (LEG_TO_LEG_DIST / 2.0) - (self.tenon_side_board.describe().thickness / 2.0);
        let y2 = cy
            - (LEG_TO_LEG_DIST / 2.0)
            - (self.leg.describe().width / 2.0)
            - MORTISE_BOARD_TENON_OVERRUN;

        scad!(Union;{
            scad!(Translate(vec3(x0, y0, z));{
                self.tenon_side_board.assemble_aligned(Axis::X),
            }),
            scad!(Translate(vec3(x0, y1, z));{
                self.tenon_side_board.assemble_aligned(Axis::X),
            }),
            scad!(Translate(vec3(x1, y2, z));{
                self.mortise_side_board.assemble_aligned(Axis::Y),
            }),
            scad!(Translate(vec3(x2, y2, z));{
                self.mortise_side_board.assemble_aligned(Axis::Y),
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
        })
    }
}
