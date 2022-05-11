use crate::{config::*, project_parts::*};
use approx::assert_relative_eq;
use parts::prelude::*;
use scad::*;

#[derive(Copy, Clone, Debug)]
pub struct HalfShelf {
    exploded_view: bool,
    front: HalfShelfBoard,
    back: HalfShelfBoard,
    supports: [ShortBoard; NUM_SHORT_BOARDS_PER_HALF_SHELF],
}

impl HalfShelf {
    pub fn new(exploded_view: bool) -> Self {
        Self {
            exploded_view,
            front: HalfShelfBoard::default(),
            back: HalfShelfBoard::default(),
            supports: [ShortBoard::default(); NUM_SHORT_BOARDS_PER_HALF_SHELF],
        }
    }

    pub fn boards(&self) -> Vec<&Board<Inch>> {
        let mut boards = vec![&self.front.0, &self.back.0];
        for b in self.supports.iter() {
            boards.push(&b.0);
        }
        boards
    }

    fn join_front_back(&self) -> ScadObject {
        let offset = BOARD_THICKNESS + SHORT_BOARD_LENGTH;

        let front = scad!(Translate(vec3(0.0, BOARD_THICKNESS.get(), 0.0));{
            scad!(Rotate(90.0, vec3(1.0, 0.0, 0.0));{
                self.front.assemble(),
            }),
        });

        let back = scad!(Translate(vec3(0.0, BOARD_THICKNESS.get(), 0.0));{
            scad!(Rotate(90.0, vec3(1.0, 0.0, 0.0));{
                self.back.assemble(),
            }),
        });

        scad!(Union;{
            back,
            scad!(Translate(vec3(0.0, offset.get(), 0.0));{
                front
            }),
        })
    }

    fn join_supports(&self) -> ScadObject {
        assert!(
            self.supports.windows(2).all(|w| w[0] == w[1]),
            "All the support boards should be the same"
        );
        assert_relative_eq!(
            BOARD_THICKNESS.get()
                + ((NUM_SHORT_BOARDS_PER_HALF_SHELF - 1) as f32)
                    * HALF_SHELF_SUPPORT_BOARD_OFFSET.get(),
            MED_BOARD_LENGTH.get()
        );

        let board = scad!(Rotate(90.0, vec3(1.0, 0.0, 0.0));{
            scad!(Rotate(90.0, vec3(0.0, 1.0, 0.0));{
                self.supports[0].assemble()
            })
        });

        let mut root = ScadObject::new(ScadElement::Union);

        for n in 0..NUM_SHORT_BOARDS_PER_HALF_SHELF {
            let offset = HALF_SHELF_SUPPORT_BOARD_OFFSET * Inch::new(n as _);
            root.add_child(scad!(Translate(vec3(offset.get(), 0.0, 0.0));{
                board.clone()
            }));
        }

        root
    }
}

impl ScadAssembler for HalfShelf {
    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.join_front_back(),
            scad!(Translate(vec3(0.0, BOARD_THICKNESS.get(), 0.0));{
                self.join_supports(),
            })
        })
    }
}
