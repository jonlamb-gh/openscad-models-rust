use dimdraw::{DimLocation, Drawing, DrawingAssembler, DrawingParams, ObjectAssembler, Viewport};
use parts::common_functions::*;
use scad::*;

use config::*;
use slab::Slab;

impl DrawingAssembler for Slab {
    fn drawing_params(&self) -> DrawingParams {
        let delta = 30.0;
        DrawingParams {
            doc_scale: 40.0,
            show_frame: true,
            doc_height: SLAB_WIDTH + 5.0,
            top_left_view_pos: vec3(-SLAB_LENGTH - delta, delta, 0.0),
            top_right_view_pos: vec3(delta * 2.5, delta, 0.0),
            bottom_left_view_pos: vec3(-SLAB_LENGTH - delta, -POST_WIDTH - delta, 0.0),
            bottom_right_view_pos: vec3(delta, -POST_THICKNESS - delta, 0.0),
        }
    }

    //fn assemble_viewport(&self, vp: Viewport) -> ScadObject {
    //}
}

impl Slab {
    /*
    fn assemble_ext_bottom_left(&self) -> ScadObject {
        let params = self.drawing_params();
        let drawing = Drawing::new(params.doc_scale);
        let mut parent = scad!(Union);
        let mut dims_children = scad!(Color(v3zero()));

        let tenon_length = SLAB_THICKNESS + TENON_OVERRUN;
        let tenon_x = POST_LENGTH - tenon_length;

        let ext_line_y = -drawing.spacing - (drawing.spacing * 3.0);

        // tenon dimension
        dims_children.add_child(
            scad!(Translate(vec3(tenon_x, -drawing.spacing * 3.0, 0.0));{
                drawing.dim_line(tenon_length, DimLocation::Left)
            }),
        );

        // extension lines
        dims_children.add_child(scad!(Translate(vec3(tenon_x, ext_line_y, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }));
        dims_children.add_child(scad!(Translate(vec3(POST_LENGTH, ext_line_y, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }));

        parent.add_child(dims_children);
        parent
    }

    fn assemble_baseline_viewport(&self, vp: Viewport) -> ScadObject {
        let params = self.drawing_params();
        let drawing = Drawing::new(params.doc_scale);
        let obj_desc = self.describe();
        let mut parent = scad!(Union);

        let mut dims_children = scad!(Color(v3zero()));

        let (major_dim, minor_dim) = match vp {
            Viewport::TopLeft => (obj_desc.length, obj_desc.width),
            // top right is used to show a perspective view, no dimensions
            Viewport::TopRight => return parent,
            Viewport::BottomLeft => (obj_desc.length, obj_desc.thickness),
            Viewport::BottomRight => (obj_desc.width, obj_desc.thickness),
        };

        // major dimension
        dims_children.add_child(
            scad!(Translate(vec3(0.0, minor_dim + (drawing.spacing * 3.0), 0.0));{
                drawing.dim_line(major_dim, DimLocation::Center)
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
            scad!(Translate(vec3(-drawing.spacing * 3.0, minor_dim, 0.0));{
                scad!(Rotate(-90.0, z_axis());{
                    drawing.dim_line(minor_dim, DimLocation::Center)
                })
            }),
        );

        // extension lines
        dims_children.add_child(
            scad!(Translate(vec3(-drawing.spacing * 3.0 - drawing.spacing, 0.0, 0.0));{
                drawing.line(drawing.spacing * 3.0, false, false)
            }),
        );
        dims_children.add_child(
            scad!(Translate(vec3(-drawing.spacing * 3.0 - drawing.spacing, minor_dim, 0.0));{
                drawing.line(drawing.spacing * 3.0, false, false)
            }),
        );

        parent.add_child(dims_children);
        parent
    }
    */
}
