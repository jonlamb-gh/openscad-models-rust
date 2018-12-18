use crate::config::*;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use nalgebra::Vector2;
use parts::common_functions::*;
use parts::Board;
use scad::*;

pub enum Quadrant {
    /// Bottom left
    Q0,
    /// Top left
    Q1,
    /// Top right
    Q2,
    /// Bottom right
    Q3,
}

qstruct!(Leg(color: Option<&'static str>) {
    board: Board = Board::from_array(&LEG_BOARD_SIZE, color),
});

impl Leg {
    pub fn assemble_aligned(&self, quad: Quadrant) -> ScadObject {
        let angle = match quad {
            Quadrant::Q0 => 0.0,
            Quadrant::Q1 => -90.0,
            Quadrant::Q2 => 180.0,
            Quadrant::Q3 => 90.0,
        };

        let obj = scad!(Translate(vec3(self.board.describe().thickness, 0.0, 0.0));{
            scad!(Rotate(-90.0, y_axis());{
                self.assemble()
            })
        });

        self.assemble_rotate(obj, angle)
    }

    // TODO - move this somewhere
    fn assemble_rotate(&self, obj: ScadObject, angle: f32) -> ScadObject {
        let dim = self.board.describe();

        scad!(Translate(vec3(dim.thickness / 2.0, dim.width / 2.0, 0.0));{
            scad!(Rotate(angle, z_axis());{
                scad!(Translate(vec3(-dim.thickness / 2.0, -dim.width / 2.0, 0.0));{
                    obj
                })
            })
        })
    }

    fn chamfer_cutaway(&self) -> ScadObject {
        let extrude_params = LinExtrudeParams {
            height: self.board.describe().thickness + (2.0 * VISUAL_OVERRUN),
            center: false,
            ..Default::default()
        };

        let points: Vec<Vector2<f32>> = vec![
            Vector2::new(-VISUAL_OVERRUN, -VISUAL_OVERRUN),
            Vector2::new(-VISUAL_OVERRUN, LEG_CHAMFER_DEPTH),
            Vector2::new(LEG_CHAMFER_LENGTH, -VISUAL_OVERRUN),
        ];

        let poly_params = PolygonParameters::new(points);

        let mut parent = scad!(LinearExtrude(extrude_params));
        parent.add_child(scad!(Polygon(poly_params)));

        parent
    }

    /// Cutout along the X axis
    fn chamfer_x_cutaway(&self) -> ScadObject {
        scad!(Translate(vec3(0.0, 0.0, -VISUAL_OVERRUN));{
            self.chamfer_cutaway()
        })
    }

    /// Cutout along the Y axis
    fn chamfer_y_cutaway(&self) -> ScadObject {
        scad!(Translate(vec3(0.0, -VISUAL_OVERRUN, self.board.describe().thickness));{
            scad!(Rotate(-90.0, x_axis());{
                self.chamfer_cutaway()
            })
        })
    }
}

impl ObjectAssembler for Leg {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        let mut parent = scad!(Difference);
        parent.add_child(self.board.assemble());
        parent.add_child(self.chamfer_x_cutaway());
        parent.add_child(self.chamfer_y_cutaway());
        parent
    }
}
