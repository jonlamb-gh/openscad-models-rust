use crate::config::*;
use crate::cutaway::Cutaway;
use dimdraw::{ObjectAssembler, ObjectDescriptor};
use nalgebra::Vector3;
use parts::common_functions::z_axis;
use parts::Board;
use scad::*;

pub enum SupportSide {
    /// Quandrants Q0 and Q1
    Left,
    /// Quandrants Q2 and Q3
    Right,
}

qstruct!(TopSupportBoard(side: SupportSide, color: Option<&'static str>) {
    board: Board = Board::from_array(&TOP_SUPPORT_BOARD_SIZE, color),
    side: SupportSide = side,
});

impl TopSupportBoard {
    // TODO - trait
    // move to Part/part.rs?
    pub fn abs_pos(side: SupportSide) -> Vector3<f32> {
        let center = Vector3::new(TOTAL_SIZE[0] / 2.0, 0.0, 0.0);

        let mw = TOP_SUPPORT_BOARD_WIDTH;
        let hlegt = (LEG_THICKNESS / 2.0) - TOP_SUPPORT_LEG_CUTOUT_DEPTH;

        let hmajor = MAJOR_LEG_TO_LEG_DIST / 2.0;

        let t = match side {
            SupportSide::Left => Vector3::new(-hmajor + hlegt, TOP_SUPPORT_BOARD_INSET, 0.0),
            SupportSide::Right => Vector3::new(hmajor - hlegt - mw, TOP_SUPPORT_BOARD_INSET, 0.0),
        };

        center + t
    }

    pub fn assemble_aligned(&self) -> ScadObject {
        scad!(Translate(vec3(0.0, self.board.describe().length, 0.0));{
            scad!(Rotate(-90.0, z_axis());{
                self.board.assemble()
            })
        })
    }

    fn leg_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            -VISUAL_OVERRUN,
            -LEG_WIDTH / 2.0,
            -VISUAL_OVERRUN,
            // size
            TOP_SUPPORT_LEG_CUTOUT_DEPTH + VISUAL_OVERRUN,
            LEG_WIDTH,
            LEG_LENGTH - SIDE_SUPPORT_BOARD_HEIGHT + VISUAL_OVERRUN + TOP_SUPPORT_CUTOUT_DEPTH,
        )
    }

    fn leg_cutaways(&self) -> ScadObject {
        let cy = TOP_SUPPORT_BOARD_LENGTH / 2.0;
        let hminor = MINOR_LEG_TO_LEG_DIST / 2.0;
        let rx = TOP_SUPPORT_BOARD_WIDTH - TOP_SUPPORT_LEG_CUTOUT_DEPTH + VISUAL_OVERRUN;

        match self.side {
            SupportSide::Left => scad!(Union;{
                    scad!(Translate(vec3(0.0, cy + hminor, 0.0));{
                        self.leg_cutaway().assemble(),
                    }),
                    scad!(Translate(vec3(0.0, cy - hminor, 0.0));{
                        self.leg_cutaway().assemble()
                    })
                }),
            SupportSide::Right => scad!(Union;{
                    scad!(Translate(vec3(rx, cy + hminor, 0.0));{
                        self.leg_cutaway().assemble(),
                    }),
                    scad!(Translate(vec3(rx, cy - hminor, 0.0));{
                        self.leg_cutaway().assemble()
                    })
                }),
        }
    }

    fn side_support_cutaway(&self) -> Cutaway {
        Cutaway::from_parts(
            // position
            -VISUAL_OVERRUN,
            -SIDE_SUPPORT_BOARD_THICKNESS / 2.0,
            -VISUAL_OVERRUN,
            // size
            TOP_SUPPORT_BOARD_WIDTH + (2.0 * VISUAL_OVERRUN),
            SIDE_SUPPORT_BOARD_THICKNESS,
            TOP_SUPPORT_CUTOUT_DEPTH + VISUAL_OVERRUN,
        )
    }

    fn side_support_cutaways(&self) -> ScadObject {
        let cy = TOP_SUPPORT_BOARD_LENGTH / 2.0;
        let hminor = MINOR_LEG_TO_LEG_DIST / 2.0;

        scad!(Union;{
            scad!(Translate(vec3(0.0, cy + hminor, 0.0));{
                self.side_support_cutaway().assemble()
            }),
            scad!(Translate(vec3(0.0, cy - hminor, 0.0));{
                self.side_support_cutaway().assemble()
            })
        })
    }

    fn bolt_head_cutouts(&self) -> ScadObject {
        // Being specific on bolt layout
        assert_eq!(TOP_BOARD_MAJOR_COUNT, 3);
        assert_eq!(TOP_BOARD_MINOR_COUNT, 2);

        let mut parent = scad!(Union);

        let bolt_head_cutout = scad!(Cylinder(
            TOP_SUPPORT_BOLT_HEAD_THICKNESS + (2.0 * VISUAL_OVERRUN),
            CircleType::Radius(TOP_SUPPORT_BOLT_HEAD_RADIUS)
        ));

        let bolt_cutout = scad!(Cylinder(
            TOP_SUPPORT_BOARD_THICKNESS + VISUAL_OVERRUN,
            CircleType::Radius(TOP_SUPPORT_BOLT_RADIUS)
        ));

        let dims = self.describe();
        let cy = dims.length / 2.0;
        let z = -VISUAL_OVERRUN;
        let x = dims.width / 2.0;

        let offsets = [
            TOP_BOARD_MAJOR_WIDTH / 4.0,
            (TOP_BOARD_MAJOR_WIDTH / 2.0) + (TOP_BOARD_MINOR_WIDTH / 2.0),
            (TOP_BOARD_MAJOR_WIDTH / 2.0) + TOP_BOARD_MINOR_WIDTH + (TOP_BOARD_MAJOR_WIDTH / 4.0),
            (TOP_BOARD_MAJOR_WIDTH / 2.0) + TOP_BOARD_MINOR_WIDTH + (TOP_BOARD_MAJOR_WIDTH / 2.0),
        ];

        for y in &offsets {
            let t = vec3(x, cy + y, z);
            parent.add_child(scad!(Translate(t);{ bolt_head_cutout.clone() }));
            parent.add_child(scad!(Translate(t);{ bolt_cutout.clone() }));
            let t = vec3(x, cy - y, z);
            parent.add_child(scad!(Translate(t);{ bolt_head_cutout.clone() }));
            parent.add_child(scad!(Translate(t);{ bolt_cutout.clone() }));
        }

        parent
    }
}

impl ObjectAssembler for TopSupportBoard {
    fn describe(&self) -> ObjectDescriptor {
        self.board.describe()
    }

    fn assemble(&self) -> ScadObject {
        scad!(Difference;{
            self.assemble_aligned(),
            self.bolt_head_cutouts(),
            self.leg_cutaways(),
            self.side_support_cutaways(),
        })
    }
}
