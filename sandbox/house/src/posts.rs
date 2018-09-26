/// Posts (Hashira)
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Board;
use scad::*;

use config::*;

qstruct!(Posts(color: Option<&'static str>) {
    l12: Board = Board::new(
        POST_L12_LENGTH,
        POST_WIDTH,
        POST_THICKNESS,
        color),
});

impl Posts {
    fn assemble_aligned(&self) -> ScadObject {
        scad!(Translate(vec3(self.l12.thickness(), 0.0, 0.0));{
            scad!(Rotate(-90.0, y_axis());{
                self.l12.assemble()
            })
        })
    }
}

impl ObjectAssembler for Posts {
    // TODO - containing volume?
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: ft_to_cm(44.0),
            width: ft_to_cm(32.0),
            thickness: ft_to_cm(50.0),
        }
    }

    fn assemble(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // col 1
        for row in 0..5 {
            let y_offset = row as f32 * ft_to_cm(8.0);
            parent.add_child(scad!(Translate(vec3(0.0, y_offset, 0.0));{
                self.assemble_aligned()
            }));
        }

        // col 2
        let x_offset = ft_to_cm(8.0);
        for row in 0..5 {
            if row == 0 || row == 3 || row == 4 {
                let y_offset = row as f32 * ft_to_cm(8.0);
                parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                    self.assemble_aligned()
                }));
            }
        }

        // col 3
        parent.add_child(scad!(Translate(vec3(ft_to_cm(16.0), ft_to_cm(32.0), 0.0));{
            self.assemble_aligned()
        }));

        // col 4
        let x_offset = ft_to_cm(24.0);
        for row in 0..5 {
            if row == 0 || row == 3 || row == 4 {
                let y_offset = row as f32 * ft_to_cm(8.0);
                parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                    self.assemble_aligned()
                }));
            }
        }

        // col 5
        let x_offset = ft_to_cm(32.0);
        for row in 0..5 {
            let y_offset = row as f32 * ft_to_cm(8.0);
            parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.assemble_aligned()
            }));
        }

        // col 6
        let x_offset = ft_to_cm(36.0);
        for row in 0..5 {
            if row == 2 || row == 3 || row == 4 {
                let y_offset = row as f32 * ft_to_cm(8.0);
                parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                    self.assemble_aligned()
                }));
            }
        }

        // col 7
        let x_offset = ft_to_cm(44.0);
        for row in 0..5 {
            if row > 0 {
                let y_offset = row as f32 * ft_to_cm(8.0);
                parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                    self.assemble_aligned()
                }));
            }
        }

        parent
    }
}
