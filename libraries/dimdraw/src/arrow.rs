use super::HEIGHT;

use scad::*;

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
