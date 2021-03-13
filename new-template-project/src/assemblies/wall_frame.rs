use approx::assert_relative_eq;
use parts::utils::{y_axis, Centimeter, Unit};
use parts::Board;
use scad::{scad, vec3, Rotate, ScadObject, Translate, Union};
use scad_assembler::ScadAssembler;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct WallFrame {
    top_and_bottom_board: Board,
    stud_board: Board,
    separation_distance: Centimeter,
}

impl WallFrame {
    pub fn new(
        top_and_bottom_board: Board,
        stud_board: Board,
        separation_distance: Centimeter,
    ) -> Self {
        assert!(separation_distance > 0.0);
        let span = top_and_bottom_board.dimensions().length() - stud_board.dimensions().thickness();
        assert_relative_eq!(span % separation_distance, 0.0);
        WallFrame {
            top_and_bottom_board,
            stud_board,
            separation_distance,
        }
    }

    fn assemble_top_and_bottom(&self) -> ScadObject {
        let offset = self.top_and_bottom_board.dimensions().thickness()
            + self.stud_board.dimensions().length();

        scad!(Union;{
            self.top_and_bottom_board.assemble(),
            scad!(Translate(vec3(0.0, 0.0, offset.get()));{
                self.top_and_bottom_board.assemble(),
            })
        })
    }

    fn assemble_studs(&self) -> ScadObject {
        let mut parent = scad!(Union);

        let num_studs = 1
            + (self.top_and_bottom_board.dimensions().length().get()
                / self.separation_distance.get()) as usize;

        let offset_z = self.top_and_bottom_board.dimensions().thickness();

        for offset_i in 0..num_studs {
            let offset = self.stud_board.dimensions().thickness()
                + (Centimeter::from(offset_i as f32) * self.separation_distance);
            parent.add_child(scad!(Rotate(-90.0, y_axis());{
                scad!(Translate(vec3(offset_z.get(), 0.0, -offset.get()));{
                    self.stud_board.assemble()
                })
            }));
        }

        parent
    }
}

impl ScadAssembler for WallFrame {
    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.assemble_top_and_bottom(),
            self.assemble_studs(),
        })
    }
}
