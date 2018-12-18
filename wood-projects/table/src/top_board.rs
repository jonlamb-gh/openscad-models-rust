use crate::config::*;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use nalgebra::Vector2;
use parts::common_functions::{x_axis, z_axis};
use parts::Board;
use scad::*;

pub enum WidthType {
    Major,
    Minor,
}

impl WidthType {
    fn array(&self) -> &[f32; 3] {
        match *self {
            WidthType::Major => &TOP_BOARD_MAJOR_SIZE,
            WidthType::Minor => &TOP_BOARD_MINOR_SIZE,
        }
    }
}

qstruct!(TopBoard(w_type: WidthType, color: Option<&'static str>) {
    board: Board = Board::from_array(w_type.array(), color),
});

impl TopBoard {
    pub fn assemble_aligned(&self) -> ScadObject {
        self.assemble()
    }

    fn chamfer_cutaway(&self) -> ScadObject {
        let extrude_params = LinExtrudeParams {
            height: self.board.describe().width + (2.0 * VISUAL_OVERRUN),
            center: false,
            ..Default::default()
        };

        let points: Vec<Vector2<f32>> = vec![
            Vector2::new(-VISUAL_OVERRUN, -VISUAL_OVERRUN),
            Vector2::new(-VISUAL_OVERRUN, TOP_BOARD_CHAMFER_DEPTH),
            Vector2::new(TOP_BOARD_CHAMFER_LENGTH, -VISUAL_OVERRUN),
        ];

        let poly_params = PolygonParameters::new(points);

        let mut parent = scad!(LinearExtrude(extrude_params));
        parent.add_child(scad!(Polygon(poly_params)));

        parent
    }

    fn chamfer_a_cutaway(&self) -> ScadObject {
        scad!(Translate(vec3(0.0, self.board.describe().width + VISUAL_OVERRUN, 0.0));{
            scad!(Rotate(90.0, x_axis());{
                self.chamfer_cutaway()
            })
        })
    }

    fn chamfer_b_cutaway(&self) -> ScadObject {
        scad!(Translate(vec3(self.describe().length, self.board.describe().width, 0.0));{
            scad!(Rotate(180.0, z_axis());{
                self.chamfer_a_cutaway()
            })
        })
    }
}

impl ObjectAssembler for TopBoard {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        let mut parent = scad!(Difference);
        parent.add_child(self.board.assemble());
        parent.add_child(self.chamfer_a_cutaway());
        parent.add_child(self.chamfer_b_cutaway());
        parent
    }
}
