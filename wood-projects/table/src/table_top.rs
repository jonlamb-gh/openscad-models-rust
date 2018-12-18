use crate::config::*;
use crate::top_board::TopBoard;
use crate::top_support_board::TopSupportBoard;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

qstruct!(TableTop(top_color: Option<&'static str>, support_color: Option<&'static str>) {
    top_board: TopBoard = TopBoard::new(top_color),
    support_board: TopSupportBoard = TopSupportBoard::new(support_color),
});

impl TableTop {
    pub fn assemble_top(&self) -> ScadObject {
        let z = self.support_board.describe().thickness;

        let mut parent = scad!(Union);
        for b in 0..TOP_BOARD_COUNT {
            let t = vec3(0.0, TOP_BOARD_WIDTH * b as f32, z);
            parent.add_child(scad!(Translate(t);{
                self.top_board.assemble_aligned(),
            }));
        }
        parent
    }

    pub fn assemble_top_support(&self) -> ScadObject {
        let cx = self.top_board.describe().length / 2.0;
        let z = 0.0;

        let y = TOP_SUPPORT_BOARD_INSET;
        let x0 = TOP_SUPPORT_BOARD_INSET;
        let x1 = cx - (LEG_TO_LEG_DIST / 2.0) - (LEG_THICKNESS / 2.0);
        let x2 = cx + (LEG_TO_LEG_DIST / 2.0) - (LEG_THICKNESS / 2.0);
        let x3 = self.top_board.describe().length
            - TOP_SUPPORT_BOARD_INSET
            - self.support_board.describe().width;

        scad!(Union;{
            scad!(Translate(vec3(x0, y, z));{
                self.support_board.assemble(),
            }),
            scad!(Translate(vec3(x1, y, z));{
                self.support_board.assemble(),
            }),
            scad!(Translate(vec3(x2, y, z));{
                self.support_board.assemble(),
            }),
            scad!(Translate(vec3(x3, y, z));{
                self.support_board.assemble(),
            }),
        })
    }
}

impl ObjectAssembler for TableTop {
    fn describe(&self) -> ObjectDescriptor {
        // TODO
        ObjectDescriptor {
            length: self.top_board.board.length(),
            width: self.top_board.board.width() * TOP_BOARD_COUNT as f32,
            thickness: self.top_board.describe().thickness
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
