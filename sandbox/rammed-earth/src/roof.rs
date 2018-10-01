use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Board;
use scad::*;

use config::*;

qstruct!(Roof(rafter_color: Option<&'static str>){
    rafter_board: Board = Board::new(RAFTER_LENGTH, RAFTER_WIDTH, RAFTER_THICKNESS, rafter_color),
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
        scad!(Rotate(-ROOF_SLOPE_ANGLE, x_axis());{
            scad!(Translate(vec3(0.0, ROOF_OVERHANG, 0.0));{
                scad!(Rotate(-90.0, z_axis());{
                    self.rafter_board.assemble()
                })
            })
        })
    }
}
