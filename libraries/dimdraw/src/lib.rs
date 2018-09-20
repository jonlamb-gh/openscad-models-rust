extern crate nalgebra as na;
extern crate scad;

use scad::*;

const WIDTH: f32 = 0.025;
const HEIGHT: f32 = 0.01;

pub fn arrow(length: f32, arr: f32) -> ScadObject {
    let polygon = Polygon(PolygonParameters::new(vec![
        vec2(0.0, 0.0),
        vec2(arr, arr / 2.0),
        vec2(length, 0.0),
        vec2(arr, -arr / 2.0),
    ]));

    let shape = scad!(polygon);

    scad!(LinearExtrude(
        LinExtrudeParams{
            height: HEIGHT,
            convexity: 2,
            .. Default::default()
        });
        shape
    )
}

// TODO - params for left/right arrows
pub fn line(length: f32, left_arrow: bool, right_arrow: bool) -> ScadObject {
    let arrow_size = WIDTH * 4.0;
    let arrow_length = arrow_size * 0.6;

    let mut obj = if left_arrow && right_arrow {
        scad!(Translate(vec3(arrow_length, -WIDTH / 2.0, 0.0));{
            scad!(Cube(vec3(length - (arrow_length * 2.0), WIDTH, HEIGHT)))
        })
    } else {
        // TODO
        scad!(Translate(vec3(0.0, -WIDTH / 2.0, 0.0));{
            scad!(Cube(vec3(length, WIDTH, HEIGHT))),
        })
    };

    if left_arrow {
        obj.add_child(arrow(arrow_length, arrow_size));
    }

    if right_arrow {
        obj.add_child(scad!(Translate(vec3(length, 0.0, 0.0));{
            scad!(Rotate(180.0, vec3(0.0, 0.0, 1.0));{
                arrow(arrow_length, arrow_size)
            })
        }));
    }

    obj
}
