// TODO - pull out the hard-coded magic scalers

use scad::*;

use super::{dim_line, line, na, DimLocation, ObjectAssembler, SPACING};

//#[derive(Clone)]
pub struct DrawingParams {
    // title block ?
    show_frame: bool,
    doc_height: f32,
    // top_view params, pos/orien
    top_left_view_pos: na::Vector3<f32>,
}

pub struct ObjectDescriptor {
    pub length: f32,
    pub width: f32,
    pub thickness: f32,
}

impl Default for DrawingParams {
    fn default() -> DrawingParams {
        DrawingParams {
            show_frame: true,
            // TODO
            doc_height: 7.0,
            top_left_view_pos: vec3(-15.0, 5.0, 0.0),
        }
    }
}

// assemble_frame() ?
pub trait DrawingAssembler: ObjectAssembler {
    fn describe_drawing(&self) -> DrawingParams;
    fn describe_object(&self) -> ObjectDescriptor;

    // TODO - param enum TopLeft, BottomLeft, etc
    fn assemble_preview(&self) -> ScadObject {
        self.assemble()
    }

    fn assemble_drawing(&self) -> ScadObject {
        let params = self.describe_drawing();

        scad!(Union;{
            scad!(Translate(params.top_left_view_pos);{
                self.assemble_preview(),
                scad!(Translate(vec3(0.0, 0.0, params.doc_height));{
                    self.assemble_top_left_view()
                })
            })
        })
    }

    fn assemble_top_left_view(&self) -> ScadObject {
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
