use crate::config::*;
use crate::cutaway::Cutaway;
use crate::quadrant::Quadrant;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use nalgebra::{Vector2, Vector3};
use parts::common_functions::*;
use parts::Board;
use scad::*;

#[derive(Copy, Clone, Debug)]
pub enum JoineryType {
    /// Type 0, fits A/Q0 and C/Q2
    JT0,
    /// Type 1, fits B/Q1 and D/Q3
    JT1,
}

qstruct!(Leg(joinery_type: JoineryType, color: Option<&'static str>) {
    board: Board = Board::from_array(&LEG_BOARD_SIZE, color),
    joinery_type: JoineryType = joinery_type,
});

impl Leg {
    // TODO - trait
    // move to Part/part.rs?
    // TODO - center
    pub fn abs_pos(quad: Quadrant, _center: bool) -> Vector3<f32> {
        let center = Vector3::new(TOTAL_SIZE[0] / 2.0, TOTAL_SIZE[1] / 2.0, 0.0);

        let ht = LEG_THICKNESS / 2.0;
        let hw = LEG_WIDTH / 2.0;

        let hmajor = MAJOR_LEG_TO_LEG_DIST / 2.0;
        let hminor = MINOR_LEG_TO_LEG_DIST / 2.0;

        // Translation from center
        let tq = match quad {
            Quadrant::Q0 => Vector3::new(-hmajor - ht, -hminor - hw, 0.0),
            Quadrant::Q1 => Vector3::new(-hmajor - ht, hminor - hw, 0.0),
            Quadrant::Q2 => Vector3::new(hmajor - ht, hminor - hw, 0.0),
            Quadrant::Q3 => Vector3::new(hmajor - ht, -hminor - hw, 0.0),
        };

        center + tq
    }

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

    fn jt0_major_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            SIDE_SUPPORT_BOARD_HEIGHT - SIDE_SUPPORT_BOARD_WIDTH,
            -VISUAL_OVERRUN,
            (LEG_THICKNESS / 2.0) - (SIDE_SUPPORT_BOARD_THICKNESS / 2.0),
            // size
            SIDE_SUPPORT_BOARD_WIDTH,
            LEG_WIDTH + (2.0 * VISUAL_OVERRUN),
            SIDE_SUPPORT_BOARD_THICKNESS,
        )
    }

    fn jt0_minor_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            SIDE_SUPPORT_BOARD_HEIGHT - SIDE_SUPPORT_BOARD_WIDTH + (SIDE_SUPPORT_BOARD_WIDTH / 2.0)
                - (SIDE_SUPPORT_TENON_WIDTH / 2.0),
            (LEG_WIDTH / 2.0) - (SIDE_SUPPORT_BOARD_THICKNESS / 2.0),
            -VISUAL_OVERRUN,
            // size
            SIDE_SUPPORT_TENON_WIDTH,
            SIDE_SUPPORT_BOARD_THICKNESS,
            LEG_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn jt0_minor_ext_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            SIDE_SUPPORT_BOARD_HEIGHT - SIDE_SUPPORT_BOARD_WIDTH,
            (LEG_WIDTH / 2.0) - (SIDE_SUPPORT_BOARD_THICKNESS / 2.0),
            -VISUAL_OVERRUN,
            // size
            SIDE_SUPPORT_TENON_WIDTH,
            SIDE_SUPPORT_BOARD_THICKNESS,
            LEG_THICKNESS / 2.0,
        )
    }

    fn jt1_major_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            SIDE_SUPPORT_BOARD_HEIGHT - SIDE_SUPPORT_BOARD_WIDTH,
            (LEG_WIDTH / 2.0) - (SIDE_SUPPORT_BOARD_THICKNESS / 2.0),
            -VISUAL_OVERRUN,
            // size
            SIDE_SUPPORT_BOARD_WIDTH,
            SIDE_SUPPORT_BOARD_THICKNESS,
            LEG_THICKNESS + (2.0 * VISUAL_OVERRUN),
        )
    }

    fn jt1_minor_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            SIDE_SUPPORT_BOARD_HEIGHT - SIDE_SUPPORT_BOARD_WIDTH + (SIDE_SUPPORT_BOARD_WIDTH / 2.0)
                - (SIDE_SUPPORT_TENON_WIDTH / 2.0),
            -VISUAL_OVERRUN,
            (LEG_THICKNESS / 2.0) - (SIDE_SUPPORT_BOARD_THICKNESS / 2.0),
            // size
            SIDE_SUPPORT_TENON_WIDTH,
            LEG_WIDTH + (2.0 * VISUAL_OVERRUN),
            SIDE_SUPPORT_BOARD_THICKNESS,
        )
    }

    fn jt1_minor_ext_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            SIDE_SUPPORT_BOARD_HEIGHT - SIDE_SUPPORT_BOARD_WIDTH,
            LEG_WIDTH / 2.0,
            (LEG_THICKNESS / 2.0) - (SIDE_SUPPORT_BOARD_THICKNESS / 2.0),
            // size
            SIDE_SUPPORT_TENON_WIDTH,
            LEG_WIDTH,
            SIDE_SUPPORT_BOARD_THICKNESS,
        )
    }

    fn major_cutaway(&self) -> Cutaway {
        match self.joinery_type {
            JoineryType::JT0 => self.jt0_major_cutaway(),
            JoineryType::JT1 => self.jt1_major_cutaway(),
        }
    }

    fn minor_cutaway(&self) -> Cutaway {
        match self.joinery_type {
            JoineryType::JT0 => self.jt0_minor_cutaway(),
            JoineryType::JT1 => self.jt1_minor_cutaway(),
        }
    }

    fn minor_ext_cutaway(&self) -> Cutaway {
        match self.joinery_type {
            JoineryType::JT0 => self.jt0_minor_ext_cutaway(),
            JoineryType::JT1 => self.jt1_minor_ext_cutaway(),
        }
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
        parent.add_child(self.major_cutaway().assemble());
        parent.add_child(self.minor_cutaway().assemble());
        parent.add_child(self.minor_ext_cutaway().assemble());
        parent
    }
}
