use scad::*;

use super::DOC_THICKNESS;
use drawing::Drawing;

impl Drawing {
    pub fn arrow(&self, length: f32, size: f32) -> ScadObject {
        let polygon = Polygon(PolygonParameters::new(vec![
            vec2(0.0, 0.0),
            vec2(size, size / 2.0),
            vec2(length, 0.0),
            vec2(size, -size / 2.0),
        ]));

        let shape = scad!(polygon);

        scad!(LinearExtrude(
            LinExtrudeParams{
                height: DOC_THICKNESS,
                convexity: 2,
                .. Default::default()
            });
            shape
        )
    }
}
