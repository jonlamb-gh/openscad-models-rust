use dimdraw::{DimLocation, Drawing, DrawingAssembler, DrawingParams, Viewport};
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

    fn assemble_viewport(&self, vp: Viewport) -> ScadObject {
        let mut parent = scad!(Union);

        parent.add_child(self.assemble_default_viewport(vp.clone()));

        if vp == Viewport::TopLeft {
            parent.add_child(self.assemble_ext_top_left());
        } else if vp == Viewport::BottomLeft {
            parent.add_child(self.assemble_ext_bottom_left());
        }

        parent
    }
}

impl Slab {
    fn assemble_ext_top_left(&self) -> ScadObject {
        let params = self.drawing_params();
        let drawing = Drawing::new(params.doc_scale);
        let mut parent = scad!(Union);
        let mut dims_children = scad!(Color(v3zero()));

        let edge_to_post_center = POST_TO_EDGE_DIST + (POST_THICKNESS / 2.0);

        let x0 = 0.0;
        let x1 = edge_to_post_center;
        let x2 = SLAB_LENGTH - x1;

        let ext_line_y = -drawing.spacing - (drawing.spacing * 3.0);

        // left side dimension
        dims_children.add_child(scad!(Translate(vec3(x0, -drawing.spacing * 3.0, 0.0));{
                drawing.dim_line(edge_to_post_center, DimLocation::Center)
            }));

        // center dimension
        dims_children.add_child(scad!(Translate(vec3(x1, -drawing.spacing * 3.0, 0.0));{
                drawing.dim_line(x2 - x1, DimLocation::Center)
            }));

        // right side dimension
        dims_children.add_child(scad!(Translate(vec3(x2, -drawing.spacing * 3.0, 0.0));{
                drawing.dim_line(edge_to_post_center, DimLocation::Center)
            }));

        // extension lines
        dims_children.add_child(scad!(Translate(vec3(x0, ext_line_y, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }));
        dims_children.add_child(scad!(Translate(vec3(x1, ext_line_y, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }));
        dims_children.add_child(scad!(Translate(vec3(x2, ext_line_y, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }));
        dims_children.add_child(scad!(Translate(vec3(SLAB_LENGTH, ext_line_y, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }));

        // post center crosses
        dims_children.add_child(scad!(Translate(vec3(x1, SLAB_WIDTH / 2.0, 0.0));{
                drawing.cross(6.0)
            }));
        dims_children.add_child(scad!(Translate(vec3(x2, SLAB_WIDTH / 2.0, 0.0));{
                drawing.cross(6.0)
            }));

        parent.add_child(dims_children);
        parent
    }

    fn assemble_ext_bottom_left(&self) -> ScadObject {
        let params = self.drawing_params();
        let drawing = Drawing::new(params.doc_scale);
        let mut parent = scad!(Union);
        let mut dims_children = scad!(Color(v3zero()));

        let edge_to_post_center = POST_TO_EDGE_DIST + (POST_THICKNESS / 2.0);

        let x1 = edge_to_post_center;
        let x2 = SLAB_LENGTH - x1;

        // post center crosses
        dims_children.add_child(scad!(Translate(vec3(x1, SLAB_THICKNESS / 2.0, 0.0));{
                drawing.cross(6.0)
            }));
        dims_children.add_child(scad!(Translate(vec3(x2, SLAB_THICKNESS / 2.0, 0.0));{
                drawing.cross(6.0)
            }));

        parent.add_child(dims_children);
        parent
    }
}
