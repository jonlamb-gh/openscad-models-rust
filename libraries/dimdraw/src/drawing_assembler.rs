use scad::*;

use super::na;
use constants::*;
use dim_line::DimLocation;
use drawing::Drawing;
use object_assembler::ObjectAssembler;

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

pub struct DrawingParams {
    // title block ?
    pub doc_scale: f32,
    pub show_frame: bool,
    pub doc_height: f32,
    // TODO - pos/orien
    pub top_left_view_pos: na::Vector3<f32>,
    pub top_right_view_pos: na::Vector3<f32>,
    pub bottom_left_view_pos: na::Vector3<f32>,
    pub bottom_right_view_pos: na::Vector3<f32>,
}

impl Default for DrawingParams {
    fn default() -> DrawingParams {
        DrawingParams {
            doc_scale: 4.0,
            show_frame: true,
            doc_height: 1.0,
            top_left_view_pos: vec3(-15.0, 5.0, 0.0),
            top_right_view_pos: vec3(7.0, 4.0, 0.0),
            bottom_left_view_pos: vec3(-15.0, -15.0, 0.0),
            bottom_right_view_pos: vec3(5.0, -15.0, 0.0),
        }
    }
}

// TODO
// trait that constructs and uses Drawing object or
// methods of drawing object?
// this is the former
pub trait DrawingAssembler: ObjectAssembler {
    // describes the drawing parameters to be used by assemble_drawing()/etc
    fn drawing_params(&self) -> DrawingParams;

    fn assemble_drawing(&self) -> ScadObject {
        let params = self.drawing_params();

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

    // currently at or below z == 0
    fn assemble_preview(&self, vp: Viewport) -> ScadObject {
        let obj_desc = self.describe();
        let obj = self.assemble();

        match vp {
            // top
            Viewport::TopLeft => scad!(Translate(vec3(0.0, 0.0, -obj_desc.thickness));{
                obj
            }),
            // 3D
            Viewport::TopRight => scad!(Translate(vec3(0.0, 0.0, -obj_desc.thickness));{
                scad!(Rotate(20.0, z_axis());{
                    scad!(Rotate(40.0, y_axis());{
                        scad!(Rotate(-50.0, x_axis());{
                            obj
                        })
                    })
                })
            }),
            // front
            Viewport::BottomLeft => scad!(Translate(vec3z());{
                    scad!(Rotate(-90.0, x_axis());{
                        obj
                    })
                }),
            // left
            Viewport::BottomRight => scad!(Translate(vec3(obj_desc.width, 0.0, 0.0));{
                    scad!(Rotate(90.0, z_axis());{
                        scad!(Rotate(90.0, y_axis());{
                            obj
                        })
                    })
                }),
        }
    }

    fn assemble_viewport(&self, vp: Viewport) -> ScadObject {
        self.assemble_default_viewport(vp)
    }

    // default impl just does some basic major/minor dimensions
    fn assemble_default_viewport(&self, vp: Viewport) -> ScadObject {
        let params = self.drawing_params();
        let drawing = Drawing::new(params.doc_scale);
        let obj_desc = self.describe();
        let mut parent = scad!(Union);

        let mut dims_children = scad!(Color(vec3z()));

        let (major_dim, minor_dim) = match vp {
            Viewport::TopLeft => (obj_desc.length, obj_desc.width),
            // top right is used to show a perspective view, no dimensions
            Viewport::TopRight => return parent,
            Viewport::BottomLeft => (obj_desc.length, obj_desc.thickness),
            Viewport::BottomRight => (obj_desc.width, obj_desc.thickness),
        };

        let major_dim_loc = if major_dim < 10.0 {
            DimLocation::Left
        } else {
            DimLocation::Center
        };

        let minor_dim_loc = if minor_dim < 10.0 {
            DimLocation::Left
        } else {
            DimLocation::Center
        };

        // major dimension
        dims_children.add_child(
            scad!(Translate(vec3(0.0, minor_dim + (drawing.spacing * 3.0), 0.0));{
                drawing.dim_line(major_dim, major_dim_loc)
            }),
        );

        // extension lines
        dims_children.add_child(
            scad!(Translate(vec3(0.0, minor_dim + drawing.spacing, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }),
        );
        dims_children.add_child(
            scad!(Translate(vec3(major_dim, minor_dim + drawing.spacing, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }),
        );

        // minor dimension
        dims_children.add_child(
            scad!(Translate(vec3(major_dim + drawing.spacing * 3.0, minor_dim, 0.0));{
                scad!(Rotate(-90.0, z_axis());{
                    drawing.dim_line(minor_dim, minor_dim_loc)
                })
            }),
        );

        // extension lines
        dims_children.add_child(
            scad!(Translate(vec3(major_dim + drawing.spacing, 0.0, 0.0));{
                drawing.line(drawing.spacing * 3.0, false, false)
            }),
        );
        dims_children.add_child(
            scad!(Translate(vec3(major_dim + drawing.spacing, minor_dim, 0.0));{
                drawing.line(drawing.spacing * 3.0, false, false)
            }),
        );

        parent.add_child(dims_children);

        parent
    }
}
