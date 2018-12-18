use crate::config::*;
use crate::leg::Leg;
use crate::table_top::TableTop;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

qstruct!(Table() {
    leg: Leg = Leg::new(Some("Peru")),
    top: TableTop = TableTop::new(Some("SaddleBrown"), Some("SandyBrown")),
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
                self.leg.assemble(),
            }),
            scad!(Translate(vec3(x0, y1, 0.0));{
                self.leg.assemble(),
            }),
            scad!(Translate(vec3(x1, y1, 0.0));{
                self.leg.assemble(),
            }),
            scad!(Translate(vec3(x1, y0, 0.0));{
                self.leg.assemble(),
            }),
        })
    }

    pub fn assemble_top(&self) -> ScadObject {
        scad!(Translate(vec3(0.0, 0.0, self.leg.describe().length));{
            self.top.assemble()
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
        })
    }
}
