use dimdraw::{DimLocation, Drawing, DrawingAssembler, DrawingParams, ObjectAssembler, Viewport};
use parts::common_functions::*;
use scad::*;

use config::*;
use post::Post;

impl DrawingAssembler for Post {
    fn drawing_params(&self) -> DrawingParams {
        let delta = 30.0;
        DrawingParams {
            doc_scale: 20.0,
            show_frame: true,
            doc_height: POST_LENGTH + 5.0,
            top_left_view_pos: vec3(-POST_LENGTH - delta, delta, 0.0),
            top_right_view_pos: vec3(delta * 2.0, delta, 0.0),
            bottom_left_view_pos: vec3(-POST_LENGTH - delta, -POST_THICKNESS - delta, 0.0),
            bottom_right_view_pos: vec3(delta, -POST_THICKNESS - delta, 0.0),
        }
    }

    fn assemble_preview(&self, vp: Viewport) -> ScadObject {
        let obj_desc = self.describe();
        let obj = self.assemble();

        let parent = match vp {
            // top
            Viewport::TopLeft => scad!(Translate(vec3(0.0, 0.0, -obj_desc.thickness));{
                obj
            }),
            // 3D
            Viewport::TopRight => scad!(Translate(vec3(0.0, 0.0, -obj_desc.thickness));{
                scad!(Rotate(20.0, z_axis());{
                    scad!(Rotate(40.0, y_axis());{
                        scad!(Rotate(-50.0, x_axis());{
                            scad!(Rotate(180.0, z_axis());{
                                obj
                            })
                        })
                    })
                })
            }),
            // front
            Viewport::BottomLeft => scad!(Translate(v3zero());{
                    scad!(Rotate(-90.0, x_axis());{
                        obj
                    })
                }),
            // right
            Viewport::BottomRight => scad!(Translate(vec3(obj_desc.width, 0.0, 0.0));{
                    scad!(Translate(vec3(0.0, obj_desc.thickness, 0.0));{
                        scad!(Rotate(90.0, z_axis());{
                            scad!(Rotate(-90.0, y_axis());{
                                obj
                            })
                        })
                    })
                }),
        };

        parent
    }

    fn assemble_viewport(&self, vp: Viewport) -> ScadObject {
        let mut parent = scad!(Union);

        parent.add_child(self.assemble_baseline_viewport(vp.clone()));

        if vp == Viewport::BottomLeft {
            parent.add_child(self.assemble_ext_bottom_left());
        } else if vp == Viewport::BottomRight {
            parent.add_child(self.assemble_ext_bottom_right());
        }

        parent
    }
}

impl Post {
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

    fn assemble_ext_bottom_right(&self) -> ScadObject {
        let params = self.drawing_params();
        let drawing = Drawing::new(params.doc_scale);
        let mut parent = scad!(Union);
        let mut dims_children = scad!(Color(v3zero()));

        let minor_x = POST_WIDTH - POST_TENON_MINOR_DEPTH;
        let minor_ext_line_y = -drawing.spacing - (drawing.spacing * 3.0);

        // minor depth dimension
        dims_children.add_child(
            scad!(Translate(vec3(minor_x, -drawing.spacing * 3.0, 0.0));{
                drawing.dim_line(POST_TENON_MINOR_DEPTH, DimLocation::Right)
            }),
        );

        // extension lines
        dims_children.add_child(scad!(Translate(vec3(minor_x, minor_ext_line_y, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }));
        dims_children.add_child(scad!(Translate(vec3(POST_WIDTH, minor_ext_line_y, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    drawing.line(drawing.spacing * 3.0, false, false)
                })
            }));

        let major_y = POST_THICKNESS - POST_TENON_MAJOR_DEPTH;
        let major_ext_line_x = POST_WIDTH + drawing.spacing;

        // major depth dimension
        dims_children.add_child(
            scad!(Translate(vec3(POST_WIDTH + drawing.spacing * 3.0, POST_THICKNESS, 0.0));{
                scad!(Rotate(-90.0, z_axis());{
                    drawing.dim_line(POST_TENON_MAJOR_DEPTH, DimLocation::Right)
                })
            }),
        );

        // extension lines
        dims_children.add_child(scad!(Translate(vec3(major_ext_line_x, major_y, 0.0));{
                drawing.line(drawing.spacing * 3.0, false, false)
            }));
        dims_children.add_child(
            scad!(Translate(vec3(major_ext_line_x, POST_THICKNESS, 0.0));{
                drawing.line(drawing.spacing * 3.0, false, false)
            }),
        );

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
}
