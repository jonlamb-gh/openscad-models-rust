// TODO - for doors, no bottom framing?

use dimdraw::{ObjectAssembler, ObjectDescriptor};
use scad::*;

use super::na;
use board::Board;
use common_functions::*;

// this can be improved
#[derive(Clone)]
pub struct CutoutFrameAt {
    pub cutout_frame: CutoutFrame,
    pub at: na::Vector3<f32>,
}

impl CutoutFrameAt {
    pub fn new(cutout_frame: CutoutFrame, at: na::Vector3<f32>) -> Self {
        Self { cutout_frame, at }
    }
}

#[derive(Clone)]
pub struct CutoutFrame {
    // size of the openening, cutout is larger to account for the frame
    // major is x
    major: f32,
    // minor is z
    minor: f32,
    frame_width: f32,
    frame_thickness: f32,
    major_centered: bool,
    minor_board: Board,
    major_board: Board,
}

impl CutoutFrame {
    pub fn new(
        major: f32,
        minor: f32,
        frame_width: f32,
        frame_thickness: f32,
        major_centered: bool,
        color: Option<&'static str>,
    ) -> Self {
        Self {
            major,
            minor,
            frame_width,
            frame_thickness,
            major_centered,
            minor_board: Board::new(minor, frame_width, frame_thickness, color),
            major_board: Board::new(
                major + (2.0 * frame_thickness),
                frame_width,
                frame_thickness,
                color,
            ),
        }
    }
}

impl ObjectAssembler for CutoutFrame {
    // aligned with walls (length == minor), describes the cutout
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: self.major + (2.0 * self.frame_thickness),
            width: self.frame_width,
            thickness: self.minor + (2.0 * self.frame_thickness),
        }
    }

    fn object_color(&self) -> Option<ScadObject> {
        self.major_board.object_color()
    }

    fn assemble(&self) -> ScadObject {
        if let Some(mut c) = self.object_color() {
            c.add_child(self.assemble_frame());
            c
        } else {
            self.assemble_frame()
        }
    }
}

impl CutoutFrame {
    pub fn assemble_cutout(&self) -> ScadObject {
        let obj = self.describe();
        if self.major_centered {
            scad!(Translate(vec3(-obj.length / 2.0, 0.0, 0.0));{
                scad!(Cube(vec3(obj.length, obj.width, obj.thickness)))
            })
        } else {
            scad!(Cube(vec3(obj.length, obj.width, obj.thickness)))
        }
    }

    fn assemble_frame(&self) -> ScadObject {
        let obj = scad!(Union;{
            self.assemble_major(),
            scad!(Translate(vec3(0.0, 0.0, self.frame_thickness + self.minor));{
                self.assemble_major()
            }),
            scad!(Translate(vec3(0.0, 0.0, self.frame_thickness));{
                self.assemble_minor()
            }),
            scad!(Translate(vec3(self.frame_thickness + self.major, 0.0, self.frame_thickness));{
                self.assemble_minor()
            }),
        });

        if self.major_centered {
            scad!(Translate(vec3(-self.describe().length / 2.0, 0.0, 0.0));{
                obj
            })
        } else {
            obj
        }
    }

    fn assemble_major(&self) -> ScadObject {
        self.major_board.assemble()
    }

    fn assemble_minor(&self) -> ScadObject {
        scad!(Translate(vec3(self.minor_board.thickness(), 0.0, 0.0));{
            scad!(Rotate(-90.0, y_axis());{
                self.minor_board.assemble()
            })
        })
    }
}
