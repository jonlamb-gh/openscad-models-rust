// TODO - pull out the hard-coded magic scalers
// should only expose R^2/vec2?
use scad::*;

// would rather use modules directly?
use super::{
    dim_line, line, na, vec3z, x_axis, y_axis, z_axis, DimLocation, ObjectAssembler, SPACING,
};

#[derive(Clone, PartialEq)]
pub enum Viewport {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Viewport {
    pub fn enumerate() -> &'static [Self] {
        &[
            Viewport::TopLeft,
            Viewport::TopRight,
            Viewport::BottomLeft,
            Viewport::BottomRight,
        ]
    }
}

//#[derive(Clone)]
pub struct DrawingParams {
    // title block ?
    pub show_frame: bool,
    pub doc_height: f32,
    // TODO - pos/orien
    pub top_left_view_pos: na::Vector3<f32>,
    pub top_right_view_pos: na::Vector3<f32>,
    pub bottom_left_view_pos: na::Vector3<f32>,
    pub bottom_right_view_pos: na::Vector3<f32>,
}

pub struct ObjectDescriptor {
    pub length: f32,
    pub width: f32,
    pub thickness: f32,
}

impl Default for DrawingParams {
    fn default() -> DrawingParams {
        DrawingParams {
            // TODO - doc scale?
            show_frame: true,
            // TODO
            doc_height: 1.0,
            top_left_view_pos: vec3(-15.0, 5.0, 0.0),
            top_right_view_pos: vec3(7.0, 4.0, 0.0),
            bottom_left_view_pos: vec3(-15.0, -15.0, 0.0),
            bottom_right_view_pos: vec3(5.0, -15.0, 0.0),
        }
    }
}

// assemble_frame() ?
pub trait DrawingAssembler: ObjectAssembler {
    fn describe_drawing(&self) -> DrawingParams;
    fn describe_object(&self) -> ObjectDescriptor;

    // currently at or below z == 0
    fn assemble_preview(&self, vp: Viewport) -> ScadObject {
        let obj_desc = self.describe_object();
        let parent = match vp {
            Viewport::TopLeft => scad!(Translate(vec3(0.0, 0.0, -obj_desc.thickness));{
                self.assemble()
            }),
            Viewport::TopRight => scad!(Translate(vec3(0.0, 0.0, -obj_desc.thickness));{
                scad!(Rotate(20.0, z_axis());{
                    scad!(Rotate(40.0, y_axis());{
                        scad!(Rotate(-30.0, x_axis());{
                            self.assemble()
                        })
                    })
                })
            }),
            Viewport::BottomLeft => scad!(Translate(vec3z());{
                    scad!(Rotate(-90.0, x_axis());{
                        self.assemble()
                    })
                }),
            Viewport::BottomRight => scad!(Translate(vec3(obj_desc.width, 0.0, 0.0));{
                    scad!(Rotate(90.0, z_axis());{
                        scad!(Rotate(90.0, y_axis());{
                            self.assemble()
                        })
                    })
                }),
        };

        parent
    }

    fn assemble_drawing(&self) -> ScadObject {
        let params = self.describe_drawing();

        let mut parent = scad!(Union);

        for vp in Viewport::enumerate() {
            let vp_offset = match vp {
                Viewport::TopLeft => params.top_left_view_pos,
                Viewport::TopRight => params.top_right_view_pos,
                Viewport::BottomLeft => params.bottom_left_view_pos,
                Viewport::BottomRight => params.bottom_right_view_pos,
            };

            parent.add_child(scad!(Translate(vp_offset);{
                    self.assemble_preview(vp.clone()),
                    scad!(Translate(vec3(0.0, 0.0, params.doc_height));{
                        self.assemble_viewport(vp.clone())
                    })
                }));
        }

        parent
    }

    // default impl just does some basic major/minor dimensions
    fn assemble_viewport(&self, vp: Viewport) -> ScadObject {
        let obj_desc = self.describe_object();
        let mut parent = scad!(Union);

        let mut dims_children = scad!(Color(vec3z()));

        let (major_dim, minor_dim) = match vp {
            Viewport::TopLeft => (obj_desc.length, obj_desc.width),
            // top right is used to show a perspective view, no dimensions
            Viewport::TopRight => return parent,
            Viewport::BottomLeft => (obj_desc.length, obj_desc.thickness),
            Viewport::BottomRight => (obj_desc.width, obj_desc.thickness),
        };

        // major dimension
        dims_children.add_child(
            scad!(Translate(vec3(0.0, minor_dim + (SPACING * 3.0), 0.0));{
                dim_line(major_dim, DimLocation::Center)
            }),
        );

        // extension lines
        dims_children.add_child(scad!(Translate(vec3(0.0, minor_dim + SPACING, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    line(SPACING * 3.0, false, false)
                })
            }));
        dims_children.add_child(scad!(Translate(vec3(major_dim, minor_dim + SPACING, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    line(SPACING * 3.0, false, false)
                })
            }));

        // minor dimension
        dims_children.add_child(
            scad!(Translate(vec3(major_dim + SPACING * 3.0, minor_dim, 0.0));{
                scad!(Rotate(-90.0, z_axis());{
                    dim_line(minor_dim, DimLocation::Center)
                })
            }),
        );

        // extension lines
        dims_children.add_child(scad!(Translate(vec3(major_dim + SPACING, 0.0, 0.0));{
                line(SPACING * 3.0, false, false)
            }));
        dims_children.add_child(scad!(Translate(vec3(major_dim + SPACING, minor_dim, 0.0));{
                line(SPACING * 3.0, false, false)
            }));

        parent.add_child(dims_children);

        parent
    }
}
