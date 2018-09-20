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
pub fn line(length: f32) -> ScadObject {
    scad!(Translate(vec3(0.0, -WIDTH / 2.0, 6.0));{
        scad!(Cube(vec3(length, WIDTH, HEIGHT))),
        arrow(WIDTH * 4.0 * 0.6, WIDTH * 4.0),
    })
}
