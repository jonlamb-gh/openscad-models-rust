use scad::*;

use super::{arrow, HEIGHT, LINE_WIDTH};

pub fn line(length: f32, left_arrow: bool, right_arrow: bool) -> ScadObject {
    assert!(length >= 0.0);

    let arrow_size = LINE_WIDTH * 4.0;
    let arrow_length = arrow_size * 0.6;

    let mut parent = scad!(Union);

    let line_obj = if left_arrow && right_arrow {
        scad!(Translate(vec3(arrow_length, -LINE_WIDTH / 2.0, 0.0));{
            scad!(Cube(vec3(length - (arrow_length * 2.0), LINE_WIDTH, HEIGHT)))
        })
    } else if left_arrow {
        scad!(Translate(vec3(arrow_length, -LINE_WIDTH / 2.0, 0.0));{
            scad!(Cube(vec3(length - arrow_length, LINE_WIDTH, HEIGHT))),
        })
    } else if right_arrow {
        scad!(Translate(vec3(0.0, -LINE_WIDTH / 2.0, 0.0));{
            scad!(Cube(vec3(length - arrow_length, LINE_WIDTH, HEIGHT))),
        })
    } else {
        scad!(Translate(vec3(0.0, -LINE_WIDTH / 2.0, 0.0));{
            scad!(Cube(vec3(length, LINE_WIDTH, HEIGHT))),
        })
    };

    parent.add_child(line_obj);

    if left_arrow {
        parent.add_child(arrow(arrow_length, arrow_size));
    }

    if right_arrow {
        parent.add_child(scad!(Translate(vec3(length, 0.0, 0.0));{
            scad!(Rotate(180.0, vec3(0.0, 0.0, 1.0));{
                arrow(arrow_length, arrow_size)
            })
        }));
    }

    parent
}
