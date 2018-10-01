use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Board;
use scad::*;

use config::*;

qstruct!(Roof(rafter_color: Option<&'static str>){
    rafter_board: Board = Board::new(
        rafter_length(RAFTER_SPAN_LENGTH),
        RAFTER_WIDTH,
        RAFTER_THICKNESS,
        rafter_color),
    rafter_major_support_board: Board = Board::new(
        RAFTER_SPAN_LENGTH,
        RAFTER_BEAM_WIDTH,
        RAFTER_BEAM_THICKNESS,
        rafter_color),
    rafter_minor_support_board: Board = Board::new(
        rafter_minor_support_length(RAFTER_SPAN_LENGTH),
        RAFTER_BEAM_WIDTH,
        RAFTER_BEAM_THICKNESS,
        rafter_color),
});

impl ObjectAssembler for Roof {
    // TODO - containing volume?
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: ft_to_cm(52.0),
            width: ft_to_cm(36.0),
            thickness: ft_to_cm(15.0),
        }
    }

    fn assemble(&self) -> ScadObject {
        self.assemble_rafters()
    }
}

impl Roof {
    fn assemble_rafters(&self) -> ScadObject {
        let mut parent = scad!(Union);

        let cnt = (ft_to_cm(52.0) / RAFTER_SEP_DISTANCE).ceil() as usize;

        for r in 0..cnt {
            let x_offset = r as f32 * RAFTER_SEP_DISTANCE;
            parent.add_child(scad!(Translate(vec3(x_offset, ft_to_cm(36.0), 0.0));{
                self.assemble_rafter()
            }));
        }

        parent
    }

    fn assemble_rafter(&self) -> ScadObject {
        let center_offset = (RAFTER_WIDTH - RAFTER_BEAM_WIDTH).abs() / 2.0;

        scad!(Union;{
            scad!(Translate(vec3(-center_offset, 0.0, RAFTER_BEAM_THICKNESS));{
                self.assemble_rafter_board()
            }),
            self.assemble_rafter_major_board(),
            scad!(Translate(vec3(0.0, -RAFTER_SPAN_LENGTH, RAFTER_BEAM_THICKNESS));{
                self.assemble_rafter_minor_board()
            }),
        })
    }

    fn assemble_rafter_board(&self) -> ScadObject {
        scad!(Rotate(-ROOF_SLOPE_ANGLE, x_axis());{
            scad!(Translate(vec3(0.0, RAFTER_OVERHANG, 0.0));{
                scad!(Rotate(-90.0, z_axis());{
                    self.rafter_board.assemble()
                })
            })
        })
    }

    fn assemble_rafter_major_board(&self) -> ScadObject {
        scad!(Rotate(-90.0, z_axis());{
            self.rafter_major_support_board.assemble()
        })
    }

    fn assemble_rafter_minor_board(&self) -> ScadObject {
        scad!(Translate(vec3(RAFTER_BEAM_WIDTH, RAFTER_BEAM_THICKNESS, 0.0));{
            scad!(Rotate(90.0, z_axis());{
                scad!(Rotate(-90.0, y_axis());{
                    self.rafter_minor_support_board.assemble()
                })
            })
        })
    }
}

// l = (span / cos(slope)) + (2 * overhang)
fn rafter_length(span: f32) -> f32 {
    (span / ROOF_SLOPE_ANGLE.to_radians().cos()) + (2.0 * RAFTER_OVERHANG)
}

// l = tan(slope-angle) * span
fn rafter_minor_support_length(span: f32) -> f32 {
    ROOF_SLOPE_ANGLE.to_radians().tan() * span
}
