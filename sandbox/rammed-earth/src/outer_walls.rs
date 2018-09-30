use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::{CutoutFrame, CutoutFrameAt, Wall};
use scad::*;

use config::*;

qstruct!(OuterWalls(frame_color: Option<&'static str>, color: Option<&'static str>) {
    window_4x4_frame: CutoutFrame = CutoutFrame::new(
        OUTER_WINDOW_SQ4_MAJOR,
        OUTER_WINDOW_SQ4_MINOR,
        OUTER_WINDOW_FRAME_WIDTH,
        OUTER_WINDOW_FRAME_THICKNESS,
        true,
        frame_color),
    window_2x2_frame: CutoutFrame = CutoutFrame::new(
        OUTER_WINDOW_SQ2_MAJOR,
        OUTER_WINDOW_SQ2_MINOR,
        OUTER_WINDOW_FRAME_WIDTH,
        OUTER_WINDOW_FRAME_THICKNESS,
        true,
        frame_color),
    window_8x2_frame: CutoutFrame = CutoutFrame::new(
        OUTER_WINDOW_8X2_MAJOR,
        OUTER_WINDOW_8X2_MINOR,
        OUTER_WINDOW_FRAME_WIDTH,
        OUTER_WINDOW_FRAME_THICKNESS,
        true,
        frame_color),
    window_6x2_frame: CutoutFrame = CutoutFrame::new(
        OUTER_WINDOW_6X2_MAJOR,
        OUTER_WINDOW_6X2_MINOR,
        OUTER_WINDOW_FRAME_WIDTH,
        OUTER_WINDOW_FRAME_THICKNESS,
        true,
        frame_color),
    window_7x5_frame: CutoutFrame = CutoutFrame::new(
        OUTER_WINDOW_7X5_MAJOR,
        OUTER_WINDOW_7X5_MINOR,
        OUTER_WINDOW_FRAME_WIDTH,
        OUTER_WINDOW_FRAME_THICKNESS,
        true,
        frame_color),
    single_door_frame: CutoutFrame = CutoutFrame::new(
        SINGLE_DOOR_MAJOR,
        SINGLE_DOOR_MINOR,
        DOOR_FRAME_WIDTH,
        DOOR_FRAME_THICKNESS,
        false,
        frame_color),
    double_door_frame: CutoutFrame = CutoutFrame::new(
        DOUBLE_DOOR_MAJOR,
        DOUBLE_DOOR_MINOR,
        DOOR_FRAME_WIDTH,
        DOOR_FRAME_THICKNESS,
        true,
        frame_color),
    l4: Wall = Wall::new(
        OUTER_WALL_L4_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!()),
    l6: Wall = Wall::new(
        OUTER_WALL_L6_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!()),
    l6_2x2_window: Wall = Wall::new(
        OUTER_WALL_L6_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            window_2x2_frame.clone(),
            vec3(OUTER_WALL_L6_LENGTH / 2.0, -DOOR_FRAME_OVERRUN, OUTER_WINDOW_H5_Z),
        ))),
    l8: Wall = Wall::new(
        OUTER_WALL_L8_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!()),
    l8_single_door: Wall = Wall::new(
        OUTER_WALL_L8_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            single_door_frame.clone(),
            vec3(ft_to_cm(2.0), -DOOR_FRAME_OVERRUN, 0.0),
        ))),
    l8_4x4_window: Wall = Wall::new(
        OUTER_WALL_L8_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            window_4x4_frame.clone(),
            vec3(ft_to_cm(4.0), -DOOR_FRAME_OVERRUN, OUTER_WINDOW_H3_Z),
        ))),
    l10: Wall = Wall::new(
        OUTER_WALL_L10_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!()),
    l10_4x4_window: Wall = Wall::new(
        OUTER_WALL_L10_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            window_4x4_frame.clone(),
            vec3(ft_to_cm(5.0), -DOOR_FRAME_OVERRUN, OUTER_WINDOW_H3_Z),
        ))),
    l10_single_door: Wall = Wall::new(
        OUTER_WALL_L10_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            single_door_frame.clone(),
            vec3(ft_to_cm(4.0), -DOOR_FRAME_OVERRUN, 0.0),
        ))),
    l10_double_door: Wall = Wall::new(
        OUTER_WALL_L10_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            double_door_frame.clone(),
            vec3(OUTER_WALL_L10_LENGTH / 2.0, -DOOR_FRAME_OVERRUN, 0.0),
        ))),
    l10_6x2_window: Wall = Wall::new(
        OUTER_WALL_L10_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            window_6x2_frame.clone(),
            vec3(OUTER_WALL_L10_LENGTH / 2.0, -DOOR_FRAME_OVERRUN, OUTER_WINDOW_H5_Z),
        ))),
    l10_2x2_window: Wall = Wall::new(
        OUTER_WALL_L10_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            window_2x2_frame.clone(),
            vec3(ft_to_cm(3.0), -DOOR_FRAME_OVERRUN, OUTER_WINDOW_H5_Z),
        ))),
    l12: Wall = Wall::new(
        OUTER_WALL_L12_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!()),
    l12_double_door: Wall = Wall::new(
        OUTER_WALL_L12_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            double_door_frame.clone(),
            vec3(OUTER_WALL_L12_LENGTH / 2.0, -DOOR_FRAME_OVERRUN, 0.0),
        ))),
    l12_8x2_window: Wall = Wall::new(
        OUTER_WALL_L12_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            window_8x2_frame.clone(),
            vec3(OUTER_WALL_L12_LENGTH / 2.0, -DOOR_FRAME_OVERRUN, OUTER_WINDOW_H6P3_Z),
        ))),
    l12_6x2_window: Wall = Wall::new(
        OUTER_WALL_L12_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            window_6x2_frame.clone(),
            vec3(OUTER_WALL_L12_LENGTH / 2.0, -DOOR_FRAME_OVERRUN, OUTER_WINDOW_H6P3_Z),
        ))),
    l12_7x5_window: Wall = Wall::new(
        OUTER_WALL_L12_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color,
        vec!(CutoutFrameAt::new(
            window_7x5_frame.clone(),
            vec3(OUTER_WALL_L12_LENGTH / 2.0, -DOOR_FRAME_OVERRUN, OUTER_WINDOW_H3_Z),
        ))),
});

impl OuterWalls {
    fn assemble_rows(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // row A
        let y_offset = 0.0;
        let x_offset = OUTER_WALL_THICKNESS;
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l10_6x2_window.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(12.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l4.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(16.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l12_double_door.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(28.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l6_2x2_window.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(34.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l10_2x2_window.assemble_xaligned()
            }));

        // row B
        let y_offset = ft_to_cm(8.0);
        let x_offset = ft_to_cm(42.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10_single_door.assemble_xaligned()
        }));

        // row E
        let y_offset = ft_to_cm(36.0) - self.l4.thickness();
        let x_offset = 0.0;
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l4.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(4.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l12_8x2_window.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(16.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l8_single_door.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(24.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l10_4x4_window.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(34.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l6.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(40.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10_4x4_window.assemble_xaligned()
        }));

        parent
    }

    fn assemble_columns(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // col 1
        let x_offset = 0.0;
        let y_offset = 0.0;
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l6.assemble_yaligned()
        }));
        let y_offset = self.l8.thickness() + ft_to_cm(4.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l12_7x5_window.assemble_yaligned()
        }));
        let y_offset = self.l8.thickness() + ft_to_cm(16.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l12_7x5_window.assemble_yaligned()
        }));
        let y_offset = self.l8.thickness() + ft_to_cm(28.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l4.assemble_yaligned()
        }));

        // col 6
        let x_offset = ft_to_cm(44.0) - self.l6.thickness();
        let y_offset = self.l8.thickness();
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l6.assemble_yaligned()
        }));

        // col 7
        let x_offset = ft_to_cm(50.0);
        let y_offset = ft_to_cm(10.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8_4x4_window.assemble_yaligned()
        }));
        let y_offset = ft_to_cm(18.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8.assemble_yaligned()
        }));
        let y_offset = ft_to_cm(26.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10_4x4_window.assemble_yaligned()
        }));

        parent
    }
}

impl ObjectAssembler for OuterWalls {
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
            self.assemble_columns(),
        })
    }
}
