// TODO - pull out the hard-coded magic scalers

use scad::*;

use super::{dim_line, line, na, DimLocation, ObjectAssembler, SPACING};

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
    show_frame: bool,
    doc_height: f32,
    // top_view params, pos/orien
    top_left_view_pos: na::Vector3<f32>,
    top_right_view_pos: na::Vector3<f32>,
    bottom_left_view_pos: na::Vector3<f32>,
    bottom_right_view_pos: na::Vector3<f32>,
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
            doc_height: 7.0,
            top_left_view_pos: vec3(-15.0, 5.0, 0.0),
            top_right_view_pos: vec3(5.0, 5.0, 0.0),
            bottom_left_view_pos: vec3(-15.0, -15.0, 0.0),
            bottom_right_view_pos: vec3(5.0, -15.0, 0.0),
        }
    }
}

// assemble_frame() ?
pub trait DrawingAssembler: ObjectAssembler {
    fn describe_drawing(&self) -> DrawingParams;
    fn describe_object(&self) -> ObjectDescriptor;

    // TODO - param enum TopLeft, BottomLeft, etc
    fn assemble_preview(&self, _vp: Viewport) -> ScadObject {
        self.assemble()
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

    fn assemble_viewport(&self, _vp: Viewport) -> ScadObject {
        let obj_desc = self.describe_object();
        let mut parent = scad!(Union);

        let mut dims_children = scad!(Color(vec3(0.0, 0.0, 0.0)));

        // length dimension
        dims_children.add_child(
            scad!(Translate(vec3(0.0, obj_desc.width + SPACING * 3.0, 0.0));{
                dim_line(obj_desc.length, DimLocation::Center)
            }),
        );

        // extension lines
        dims_children.add_child(scad!(Translate(vec3(0.0, obj_desc.width + SPACING, 0.0));{
                scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0));{
                    line(SPACING * 3.0, false, false)
                })
            }));
        dims_children.add_child(
            scad!(Translate(vec3(obj_desc.length + SPACING, obj_desc.width + SPACING, 0.0));{
                scad!(Rotate(90.0, vec3(0.0, 0.0, 1.0));{
                    line(SPACING * 3.0, false, false)
                })
            }),
        );

        // width dimension
        dims_children.add_child(
            scad!(Translate(vec3(obj_desc.length + SPACING * 3.0, obj_desc.width, 0.0));{
                scad!(Rotate(-90.0, vec3(0.0, 0.0, 1.0));{
                    dim_line(obj_desc.width, DimLocation::Center)
                })
            }),
        );

        // extension lines
        dims_children.add_child(scad!(Translate(vec3(obj_desc.length + SPACING, 0.0, 0.0));{
                line(SPACING * 3.0, false, false)
            }));
        dims_children.add_child(
            scad!(Translate(vec3(obj_desc.length + SPACING, obj_desc.width, 0.0));{
                line(SPACING * 3.0, false, false)
            }),
        );

        parent.add_child(dims_children);

        parent
    }
}
