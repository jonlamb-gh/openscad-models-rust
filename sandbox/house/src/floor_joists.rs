/// Floor Joists
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Board;
use scad::*;

use config::*;

qstruct!(FloorJoists(color: Option<&'static str>) {
    board: Board = Board::new(
        FLOOR_JOIST_LENGTH,
        FLOOR_JOIST_WIDTH,
        FLOOR_JOIST_THICKNESS,
        color),
});

impl FloorJoists {
    // centered
    fn assemble_aligned(&self) -> ScadObject {
        scad!(Translate(vec3(-self.board.thickness() / 2.0, 0.0, 0.0));{
            scad!(Rotate(90.0, z_axis());{
                scad!(Rotate(90.0, x_axis());{
                    self.board.assemble()
                })
            })
        })
    }
}

impl ObjectAssembler for FloorJoists {
    // TODO - containing volume?
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
        /*
        ObjectDescriptor {
            length: ft_to_cm(44.0),
            width: ft_to_cm(32.0),
            thickness: ft_to_cm(50.0),
        }
        */
    }

    fn assemble(&self) -> ScadObject {
        let mut parent = scad!(Union);

        let delta: f32 = in_to_cm(16.0);

        // row A
        let y_offset = GIRDER_BEAM_WIDTH;
        let x_offset = GIRDER_BEAM_WIDTH / 2.0 + delta;
        for j in 0..((32 * 12) / 16 - 1) {
            let x_offset = x_offset + (j as f32 * delta);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.assemble_aligned()
            }));
        }

        // row B
        let y_offset = ft_to_cm(8.0) + GIRDER_BEAM_WIDTH;
        let x_offset = GIRDER_BEAM_WIDTH / 2.0 + delta;
        for j in 0..((44 * 12) / 16 - 1) {
            let x_offset = x_offset + (j as f32 * delta);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.assemble_aligned()
            }));
        }

        // row C
        let y_offset = ft_to_cm(16.0) + GIRDER_BEAM_WIDTH;
        let x_offset = GIRDER_BEAM_WIDTH / 2.0 + delta;
        for j in 0..((44 * 12) / 16 - 1) {
            let x_offset = x_offset + (j as f32 * delta);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.assemble_aligned()
            }));
        }

        // row D
        let y_offset = ft_to_cm(24.0) + GIRDER_BEAM_WIDTH;
        let x_offset = GIRDER_BEAM_WIDTH / 2.0 + delta;
        for j in 0..((44 * 12) / 16 - 1) {
            let x_offset = x_offset + (j as f32 * delta);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.assemble_aligned()
            }));
        }

        parent
    }
}
