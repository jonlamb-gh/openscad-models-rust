use scad::*;

use constants::z_axis;
use drawing::Drawing;

impl Drawing {
    pub fn cross(&self, size: f32) -> ScadObject {
        scad!(Union;{
            scad!(Translate(vec3(-size / 2.0, 0.0, 0.0));{
                self.line(size, false, false)
            }),
            scad!(Translate(vec3(0.0, -size / 2.0, 0.0));{
                scad!(Rotate(90.0, z_axis());{
                    self.line(size, false, false)
                }),
            }),
        })
    }
}
