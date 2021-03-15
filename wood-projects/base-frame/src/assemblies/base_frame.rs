use crate::{boards::*, config::*, joinery::*};
use approx::{assert_relative_eq, relative_ne};
use parts::prelude::*;
use scad::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct BaseFrame {
    // TODO - don't need each board, some are the same, just needed for doing a summary
    left_long_frame_board: Board,
    right_long_frame_board: Board,
    top_short_frame_board: Board,
    bottom_short_frame_board: Board,
    slat_boards: [Board; NUM_SLAT_BOARDS],
}

impl BaseFrame {
    pub fn new() -> Self {
        BaseFrame {
            left_long_frame_board: long_frame_board(),
            right_long_frame_board: long_frame_board(),
            top_short_frame_board: short_frame_board(),
            bottom_short_frame_board: short_frame_board(),
            slat_boards: [slat_board(); NUM_SLAT_BOARDS],
        }
    }

    fn join_frame_boards(&self) -> ScadObject {
        let offset = SHORT_FRAME_BOARD_LENGTH - FRAME_BOARD_WIDTH;

        // TODO - helper method to distribute slat boards, used here to do the
        // cutouts in the frame boards
        scad!(Union;{
            self.left_long_frame_board.assemble_with(|obj, color| {
                let obj = cut_bottom_ends(self.left_long_frame_board.dimensions(), obj);
                let obj = cut_slat_board_slots(self.left_long_frame_board.dimensions(), obj);
                color_or_render_root(obj, color)
            }),
            scad!(Translate(vec3(0.0, offset.get(), 0.0));{
                self.right_long_frame_board.assemble_with(|obj, color| {
                    let obj = cut_bottom_ends(self.right_long_frame_board.dimensions(), obj);
                    let obj = cut_slat_board_slots(self.right_long_frame_board.dimensions(), obj);
                    color_or_render_root(obj, color)
                }),
            }),
            scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0));{
                scad!(Translate(vec3(0.0, -FRAME_BOARD_WIDTH.get(), 0.0));{
                    self.top_short_frame_board.assemble_with(|obj, color| {
                        let obj = cut_top_ends(self.top_short_frame_board.dimensions(), obj);
                        color_or_render_root(obj, color)
                    })
                })
            }),
            scad!(Translate(vec3(LONG_FRAME_BOARD_LENGTH.get(), 0.0, 0.0));{
                scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0));{
                    self.bottom_short_frame_board.assemble_with(|obj, color| {
                        let obj = cut_top_ends(self.bottom_short_frame_board.dimensions(), obj);
                        color_or_render_root(obj, color)
                    })
                })
            })
        })
    }

    fn join_slat_boards(&self) -> ScadObject {
        assert_relative_eq!(SLAT_BOARD_LENGTH, SHORT_FRAME_BOARD_LENGTH);
        assert!(
            self.slat_boards.windows(2).all(|w| w[0] == w[1]),
            "All the slat boards should be the same"
        );
        let board = self.slat_boards[0];
        let mut root = ScadObject::new(ScadElement::Union);
        let start = LONG_FRAME_BOARD_LENGTH / 2.0;
        let slat_offsets = slat_offsets();

        // See if distance between last slat board and inner side of the short frame board
        // matches the SLAT_BOARD_SEP_DISTANCE
        let last_offset = *slat_offsets.last().unwrap();
        let last_board_end = last_offset + board.dimensions().width();
        let dist = (LONG_FRAME_BOARD_LENGTH / 2.0) - FRAME_BOARD_WIDTH - last_board_end;
        if relative_ne!(dist, SLAT_BOARD_SEP_DISTANCE) {
            println!(
                "WARN: Last slat remaining dist={}, SLAT_BOARD_SEP_DISTANCE={}",
                dist, SLAT_BOARD_SEP_DISTANCE
            );
        }

        let obj = scad!(Translate(vec3(board.dimensions().width().get(), 0.0, 0.0));{
            scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0));{
                board.assemble_with(|obj, color| {
                    color_or_render_root(obj, color)
                })
            })
        });
        for rel_offset in slat_offsets.into_iter() {
            let offset_a = start + rel_offset;
            let offset_b = start - board.dimensions().width() - rel_offset;
            root.add_child(scad!(Translate(vec3(offset_a.get(), 0.0, 0.0));{
                obj.clone()
            }));
            root.add_child(scad!(Translate(vec3(offset_b.get(), 0.0, 0.0));{
                obj.clone()
            }));
        }

        root
    }
}

impl ScadAssembler for BaseFrame {
    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.join_frame_boards(),
            scad!(Translate(vec3(0.0, 0.0, (FRAME_BOARD_THICKNESS - SLAT_BOARD_THICKNESS).get()));{
                self.join_slat_boards(),
            })
        })
    }
}
