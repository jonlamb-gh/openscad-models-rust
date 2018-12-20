use crate::config::*;
use crate::top_board::{TopBoard, WidthType};
use crate::top_support_board::TopSupportBoard;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

qstruct!(TableTop(top_color: Option<&'static str>, support_color: Option<&'static str>) {
    top_major_board: TopBoard = TopBoard::new(WidthType::Major, top_color),
    top_minor_board: TopBoard = TopBoard::new(WidthType::Minor, top_color),
    support_board: TopSupportBoard = TopSupportBoard::new(support_color),
});

impl TableTop {
    pub fn assemble_top(&self) -> ScadObject {
        let z = self.support_board.describe().thickness;

        let mut parent = scad!(Union);
        let mut toggle = true;
        let mut y = 0.0;
        for _b in 0..TOP_BOARD_COUNT {
            let child = if toggle {
                &self.top_major_board
            } else {
                &self.top_minor_board
            };
            toggle = !toggle;

            let t = vec3(0.0, y, z);
            parent.add_child(scad!(Translate(t);{
                child.assemble_aligned(),
            }));

            y += child.describe().width;
        }
        parent
    }

    pub fn assemble_top_support(&self) -> ScadObject {
        let t_left = TopSupportBoard::abs_pos(true);
        let t_right = TopSupportBoard::abs_pos(false);

        scad!(Union;{
            scad!(Translate(t_left);{
                self.support_board.assemble(),
            }),
            scad!(Translate(t_right);{
                self.support_board.assemble(),
            }),
        })
    }
}

impl ObjectAssembler for TableTop {
    fn describe(&self) -> ObjectDescriptor {
        // TODO
        ObjectDescriptor {
            length: self.top_major_board.board.length(),
            width: TOP_TOTAL_WIDTH,
            thickness: self.top_major_board.describe().thickness
                + self.support_board.describe().thickness,
        }
    }

    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.assemble_top(),
            self.assemble_top_support(),
        })
    }
}
