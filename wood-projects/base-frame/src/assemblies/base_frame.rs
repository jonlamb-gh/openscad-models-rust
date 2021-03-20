use crate::{boards::*, config::*, joinery::*, project_parts::*};
use approx::{assert_relative_eq, relative_ne};
use parts::prelude::*;
use scad::*;

// TODO
// Part/Assembly trait
//  - part name
//  - impls assembler
//  - so the CLI can generate each part scad in a dir, same for stl files
//  - CLI flags for exploded view
//  - stuff in join_frame_boards() can be part-specific
//
//  - consider migrating the Assembler trait to Rust-Scad crate

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct BaseFrame {
    exploded_view: bool,
    left_long_frame_board: LongFrameBoard,
    right_long_frame_board: LongFrameBoard,
    top_short_frame_board: ShortFrameBoard,
    bottom_short_frame_board: ShortFrameBoard,
    slat_boards: [Board; NUM_SLAT_BOARDS],
}

impl BaseFrame {
    pub fn new(exploded_view: bool) -> Self {
        BaseFrame {
            exploded_view,
            left_long_frame_board: LongFrameBoard::new(),
            right_long_frame_board: LongFrameBoard::new(),
            top_short_frame_board: ShortFrameBoard::new(),
            bottom_short_frame_board: ShortFrameBoard::new(),
            slat_boards: [slat_board(); NUM_SLAT_BOARDS],
        }
    }

    pub fn boards(&self) -> Vec<&Board> {
        let mut boards = Vec::new();
        boards.push(&self.left_long_frame_board.0);
        boards.push(&self.right_long_frame_board.0);
        boards.push(&self.top_short_frame_board.0);
        boards.push(&self.bottom_short_frame_board.0);
        for b in self.slat_boards.iter() {
            boards.push(b);
        }
        boards
    }

    fn join_frame_boards(&self) -> ScadObject {
        let offset = SHORT_FRAME_BOARD_LENGTH - FRAME_BOARD_WIDTH;
        let exploded_view_offset = if self.exploded_view {
            EXPLODED_VIEW_OFFSET
        } else {
            0.0.into()
        };

        scad!(Union;{
            self.left_long_frame_board.assemble(),
            scad!(Translate(vec3(0.0, offset.get(), 0.0));{
                self.right_long_frame_board.assemble()
            }),
            scad!(Translate(vec3(FRAME_BOARD_WIDTH.get() - exploded_view_offset.get(), 0.0, 0.0));{
                scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0));{
                    self.top_short_frame_board.assemble()
                })
            }),
            scad!(Translate(vec3(LONG_FRAME_BOARD_LENGTH.get() + exploded_view_offset.get(), 0.0, 0.0));{
                scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0));{
                    self.bottom_short_frame_board.assemble()
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
        let exploded_view_offset = if self.exploded_view {
            EXPLODED_VIEW_OFFSET
        } else {
            0.0.into()
        };
        scad!(Union;{
            self.join_frame_boards(),
            scad!(Translate(vec3(0.0, 0.0, (FRAME_BOARD_THICKNESS - SLAT_BOARD_THICKNESS).get() + exploded_view_offset.get()));{
                self.join_slat_boards(),
            })
        })
    }
}
