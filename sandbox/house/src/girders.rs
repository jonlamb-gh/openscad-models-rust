/// Girders (Obiki)
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Board;
use scad::*;

use config::*;

qstruct!(Girders(color: Option<&'static str>) {
    l4: Board = Board::new(
        GIRDER_BEAM_L4_LENGTH,
        GIRDER_BEAM_WIDTH,
        GIRDER_BEAM_THICKNESS,
        color),
    l8: Board = Board::new(
        GIRDER_BEAM_L8_LENGTH,
        GIRDER_BEAM_WIDTH,
        GIRDER_BEAM_THICKNESS,
        color),
    l16: Board = Board::new(
        GIRDER_BEAM_L16_LENGTH,
        GIRDER_BEAM_WIDTH,
        GIRDER_BEAM_THICKNESS,
        color),
    l32: Board = Board::new(
        GIRDER_BEAM_L32_LENGTH,
        GIRDER_BEAM_WIDTH,
        GIRDER_BEAM_THICKNESS,
        color),
});

impl Girders {
    fn assemble_rows(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // row A
        parent.add_child(self.l8.assemble());
        let x_offset = ft_to_cm(8.0);
        parent.add_child(scad!(Translate(vec3(x_offset, 0.0, 0.0));{
                self.l16.assemble()
            }));
        let x_offset = ft_to_cm(24.0);
        parent.add_child(scad!(Translate(vec3(x_offset, 0.0, 0.0));{
                self.l8.assemble()
            }));

        // row B and C
        for row in 0..2 {
            let y_offset = ft_to_cm(8.0) + (row as f32 * ft_to_cm(8.0));
            parent.add_child(scad!(Translate(vec3(0.0, y_offset, 0.0));{
                    self.l32.assemble()
                }));
            let x_offset = ft_to_cm(32.0);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                    self.l4.assemble()
                }));
            let x_offset = ft_to_cm(36.0);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                    self.l8.assemble()
                }));
        }

        // row D
        let y_offset = ft_to_cm(24.0);
        parent.add_child(scad!(Translate(vec3(0.0, y_offset, 0.0));{
                self.l8.assemble()
            }));
        let x_offset = ft_to_cm(8.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l16.assemble()
            }));
        let x_offset = ft_to_cm(24.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l8.assemble()
            }));
        let x_offset = ft_to_cm(32.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l4.assemble()
            }));
        let x_offset = ft_to_cm(36.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l8.assemble()
            }));

        // row E
        let y_offset = ft_to_cm(32.0);
        for col in 0..4 {
            let x_offset = col as f32 * ft_to_cm(8.0);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                    self.l8.assemble()
                }));
        }
        let x_offset = ft_to_cm(32.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l4.assemble()
            }));
        let x_offset = ft_to_cm(36.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l8.assemble()
            }));

        parent
    }

    fn assemble_columns(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // col 1
        let x_offset = self.l8.width();
        for row in 0..4 {
            let y_offset = row as f32 * ft_to_cm(8.0);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    self.l8.assemble()
                })
            }));
        }

        // col 5
        let x_offset = self.l8.width() + ft_to_cm(32.0);
        parent.add_child(scad!(Translate(vec3(x_offset, 0.0, 0.0));{
            scad!(Rotate(90.0, z_axis());{
                self.l8.assemble()
            })
        }));

        // col 7
        let x_offset = self.l8.width() + ft_to_cm(44.0);
        for row in 0..3 {
            let y_offset = ft_to_cm(8.0) + row as f32 * ft_to_cm(8.0);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    self.l8.assemble()
                })
            }));
        }

        parent
    }
}

impl ObjectAssembler for Girders {
    // TODO - containing volume?
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: ft_to_cm(44.0),
            width: ft_to_cm(32.0),
            thickness: ft_to_cm(50.0),
        }
    }

    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.assemble_rows(),
            self.assemble_columns()
        })
    }
}
