use crate::{boards::*, config::*, joinery::*};
use parts::prelude::*;
use scad::*;
use scad_assembler::ScadAssembler;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct BaseFrame {
    left_long_frame_board: Board,
    right_long_frame_board: Board,
    top_short_frame_board: Board,
    bottom_short_frame_board: Board,
}

impl BaseFrame {
    pub fn new() -> Self {
        BaseFrame {
            left_long_frame_board: long_frame_board(),
            right_long_frame_board: long_frame_board(),
            top_short_frame_board: short_frame_board(),
            bottom_short_frame_board: short_frame_board(),
        }
    }

    fn join_frame_boards(&self) -> ScadObject {
        let offset = SHORT_FRAME_BOARD_LENGTH - FRAME_BOARD_WIDTH;

        let left_long_frame_board = cut_45deg_ends(&self.left_long_frame_board);
        let right_long_frame_board = cut_45deg_ends(&self.right_long_frame_board);

        let top_short_frame_board = cut_45deg_ends(&self.top_short_frame_board);
        let bottom_short_frame_board = cut_45deg_ends(&self.bottom_short_frame_board);

        scad!(Union;{
            left_long_frame_board,
            scad!(Translate(vec3(0.0, offset.get(), 0.0));{
                scad!(Translate(vec3(0.0, FRAME_BOARD_WIDTH.get(), FRAME_BOARD_THICKNESS.get()));{
                    scad!(Rotate(180.0, vec3(1.0, 0.0, 0.0));{
                        right_long_frame_board
                    })
                })
            }),
            scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0));{
                scad!(Translate(vec3(0.0, 0.0, FRAME_BOARD_THICKNESS.get()));{
                    scad!(Rotate(180.0, vec3(1.0, 0.0, 0.0));{
                        top_short_frame_board
                    })
                })
            }),
            scad!(Translate(vec3(LONG_FRAME_BOARD_LENGTH.get(), 0.0, 0.0));{
                scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0));{
                    bottom_short_frame_board
                })
            })
        })
    }

    // slat boards

    /*
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
    */
}

impl ScadAssembler for BaseFrame {
    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.join_frame_boards(),
            //self.assemble_studs(),
        })
    }
}
