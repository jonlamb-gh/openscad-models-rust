use crate::{assemblies::*, config::*, project_parts::*};
use approx::assert_relative_eq;
use parts::prelude::*;
use scad::*;

#[derive(Copy, Clone, Debug)]
pub struct Shelves {
    exploded_view: bool,

    // Layer 0, bottom
    l0_hs: HalfShelf,

    // Layer 1
    l1_hs: HalfShelf,
    l1_fs: FullShelf,

    // Layer 2
    l2_hs: HalfShelf,

    // Layer 3
    l3_fs_a: FullShelf,
    l3_fs_b: FullShelf,

    // Layer 4, top
    l4_fs_a: FullShelf,
    l4_fs_b: FullShelf,

    // All symmetrical
    vertical_supports: [VerticalSupportBoard; NUM_VERTICAL_SUPPORTS],
}

impl Shelves {
    pub fn new(exploded_view: bool) -> Self {
        Self {
            exploded_view,
            l0_hs: HalfShelf::new(exploded_view),
            l1_hs: HalfShelf::new(exploded_view),
            l1_fs: FullShelf::new(exploded_view),
            l2_hs: HalfShelf::new(exploded_view),
            l3_fs_a: FullShelf::new(exploded_view),
            l3_fs_b: FullShelf::new(exploded_view),
            l4_fs_a: FullShelf::new(exploded_view),
            l4_fs_b: FullShelf::new(exploded_view),
            vertical_supports: [
                VerticalSupportBoard::new(VerticalSupportBoardScrewPattern::A),
                VerticalSupportBoard::new(VerticalSupportBoardScrewPattern::B),
                VerticalSupportBoard::new(VerticalSupportBoardScrewPattern::C),
                VerticalSupportBoard::new(VerticalSupportBoardScrewPattern::D),
            ],
        }
    }

    pub fn boards(&self) -> Vec<&Board<Inch>> {
        let mut boards = Vec::new();
        boards.extend_from_slice(&self.l0_hs.boards());
        boards.extend_from_slice(&self.l1_hs.boards());
        boards.extend_from_slice(&self.l1_fs.boards());
        boards.extend_from_slice(&self.l2_hs.boards());
        boards.extend_from_slice(&self.l3_fs_a.boards());
        boards.extend_from_slice(&self.l3_fs_b.boards());
        boards.extend_from_slice(&self.l4_fs_a.boards());
        boards.extend_from_slice(&self.l4_fs_b.boards());
        for b in self.vertical_supports.iter() {
            boards.push(&b.0);
        }
        boards
    }

    fn layer0(&self) -> ScadObject {
        let ho = HORIZONTAL_SECTION_LENGTHS[0] + HORIZONTAL_SECTION_LENGTHS[1];
        scad!(Translate(vec3(ho.get(), 0.0, 0.0));{
            self.l0_hs.assemble()
        })
    }

    fn layer1(&self) -> ScadObject {
        let ho = HORIZONTAL_SECTION_LENGTHS[0];
        let a = self.l1_hs.assemble();
        let b = scad!(Translate(vec3(ho.get(), 0.0, 0.0));{
            self.l1_fs.assemble()
        });
        scad!(Union;{
            a,
            b,
        })
    }

    fn layer2(&self) -> ScadObject {
        self.l2_hs.assemble()
    }

    fn layer3(&self) -> ScadObject {
        let ho = HORIZONTAL_SECTION_LENGTHS[0] + (HORIZONTAL_SECTION_LENGTHS[1] / 2.0);
        let a = self.l3_fs_a.assemble();
        let b = scad!(Translate(vec3(ho.get(), 0.0, 0.0));{
            self.l3_fs_b.assemble()
        });
        scad!(Union;{
            a,
            b,
        })
    }

    fn layer4(&self) -> ScadObject {
        let ho = HORIZONTAL_SECTION_LENGTHS[0] + (HORIZONTAL_SECTION_LENGTHS[1] / 2.0);
        let a = self.l4_fs_a.assemble();
        let b = scad!(Translate(vec3(ho.get(), 0.0, 0.0));{
            self.l4_fs_b.assemble()
        });
        scad!(Union;{
            a,
            b,
        })
    }

    fn vertical_supports(&self) -> ScadObject {
        assert!(
            self.vertical_supports
                .map(|s| s.0)
                .windows(2)
                .all(|w| w[0] == w[1]),
            "All the support boards should be the same"
        );

        let boards: Vec<_> = self
            .vertical_supports
            .iter()
            .map(|s| {
                scad!(Rotate(-90.0, vec3(0.0, 1.0, 0.0));{
                    scad!(Rotate(-90.0, vec3(1.0, 0.0, 0.0));{
                        s.assemble()
                    })
                })
            })
            .collect();

        let oc = BOARD_WIDTH / 2.0;
        let ho = [
            HORIZONTAL_SECTION_LENGTHS[0] - oc,
            HORIZONTAL_SECTION_LENGTHS[0] + HORIZONTAL_SECTION_LENGTHS[1] - oc,
            HORIZONTAL_SECTION_LENGTHS[0]
                + HORIZONTAL_SECTION_LENGTHS[1]
                + HORIZONTAL_SECTION_LENGTHS[2]
                - BOARD_WIDTH,
        ];

        scad!(Union;{
            boards[0].clone(),
            scad!(Translate(vec3(ho[0].get(), 0.0, 0.0));{
                boards[1].clone()
            }),
            scad!(Translate(vec3(ho[1].get(), 0.0, 0.0));{
                boards[2].clone()
            }),
            scad!(Translate(vec3(ho[2].get(), 0.0, 0.0));{
                boards[3].clone()
            }),
        })
    }
}

impl ScadAssembler for Shelves {
    fn assemble(&self) -> ScadObject {
        let vo = vertical_shelf_accum_heights();
        assert_relative_eq!(
            vo[4] + BOARD_WIDTH + PARTICLE_BOARD_THICKNESS,
            LONG_BOARD_LENGTH
        );

        let vs_o = (BOARD_THICKNESS * 2.0) + SHORT_BOARD_LENGTH;

        scad!(Union;{
            scad!(Translate(vec3(0.0, 0.0, vo[0].get()));{
                self.layer0()
            }),
            scad!(Translate(vec3(0.0, 0.0, vo[1].get()));{
                self.layer1()
            }),
            scad!(Translate(vec3(0.0, 0.0, vo[2].get()));{
                self.layer2()
            }),
            scad!(Translate(vec3(0.0, 0.0, vo[3].get()));{
                self.layer3()
            }),
            scad!(Translate(vec3(0.0, 0.0, vo[4].get()));{
                self.layer4()
            }),
            scad!(Translate(vec3(0.0, vs_o.get(), 0.0));{
                self.vertical_supports()
            }),
        })
    }
}
