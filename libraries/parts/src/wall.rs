// TODO:
// - safety related assertions on cutouts/etc

use dimdraw::{some_color, ObjectAssembler, ObjectDescriptor};
use scad::*;

use common_functions::*;
use cutout_frame::CutoutFrameAt;

pub struct Wall {
    size: [f32; 3],
    color: Option<String>,
    cutout_frames: Vec<CutoutFrameAt>,
}

impl Wall {
    pub fn new(
        length: f32,
        width: f32,
        thickness: f32,
        color: Option<&'static str>,
        cutout_frames: Vec<CutoutFrameAt>,
    ) -> Self {
        Self {
            size: [length, width, thickness],
            color: some_color(color),
            cutout_frames,
        }
    }

    pub fn from_array(
        size: &[f32; 3],
        color: Option<&'static str>,
        cutout_frames: Vec<CutoutFrameAt>,
    ) -> Self {
        Self::new(size[0], size[1], size[2], color, cutout_frames)
    }

    pub fn length(&self) -> f32 {
        self.size[0]
    }

    pub fn width(&self) -> f32 {
        self.size[1]
    }

    pub fn thickness(&self) -> f32 {
        self.size[2]
    }
}

impl ObjectAssembler for Wall {
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: self.length(),
            width: self.width(),
            thickness: self.thickness(),
        }
    }

    fn object_color(&self) -> Option<ScadObject> {
        if let Some(ref c) = self.color {
            Some(scad!(NamedColor(c.to_string())))
        } else {
            None
        }
    }

    fn assemble(&self) -> ScadObject {
        let cutout_walls = scad!(Difference;{
            self.assemble_base_wall(),
            self.assemble_cutouts(),
        });

        scad!(Union;{
            cutout_walls,
            self.assemble_frames(),
        })
    }
}

impl Wall {
    fn assemble_base_wall(&self) -> ScadObject {
        if let Some(mut c) = self.object_color() {
            c.add_child(scad!(Cube(vec3(
                self.length(),
                self.thickness(),
                self.width(),
            ))));
            c
        } else {
            scad!(Cube(vec3(self.length(), self.thickness(), self.width())))
        }
    }

    fn assemble_cutouts(&self) -> ScadObject {
        let mut parent = scad!(Union);

        for c in self.cutout_frames.iter() {
            parent.add_child(scad!(Translate(c.at);{
                c.cutout_frame.assemble_cutout()
            }));
        }

        parent
    }

    fn assemble_frames(&self) -> ScadObject {
        let mut parent = scad!(Union);

        for c in self.cutout_frames.iter() {
            parent.add_child(scad!(Translate(c.at);{
                c.cutout_frame.assemble()
            }));
        }

        parent
    }

    pub fn assemble_xaligned(&self) -> ScadObject {
        self.assemble()
    }

    pub fn assemble_cxaligned(&self) -> ScadObject {
        scad!(Translate(vec3(0.0, -self.thickness() / 2.0, 0.0));{
            self.assemble_xaligned()
        })
    }

    pub fn assemble_yaligned(&self) -> ScadObject {
        scad!(Translate(vec3(self.thickness(), 0.0, 0.0));{
            scad!(Rotate(90.0, z_axis());{
                self.assemble()
            })
        })
    }

    pub fn assemble_cyaligned(&self) -> ScadObject {
        scad!(Translate(vec3(-self.thickness() / 2.0, 0.0, 0.0));{
            self.assemble_yaligned()
        })
    }
}
