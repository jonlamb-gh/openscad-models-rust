use scad::*;

use super::DOC_THICKNESS;
use drawing::Drawing;

impl Drawing {
    pub fn line(&self, length: f32, left_arrow: bool, right_arrow: bool) -> ScadObject {
        assert!(length >= 0.0);

        let arrow_size = self.line_width * 4.0;
        let arrow_length = arrow_size * 0.6;

        let mut parent = scad!(Union);

        let line_obj = if left_arrow && right_arrow {
            scad!(Translate(vec3(arrow_length, -self.line_width / 2.0, 0.0));{
                scad!(Cube(vec3(length - (arrow_length * 2.0), self.line_width, DOC_THICKNESS)))
            })
        } else if left_arrow {
            scad!(Translate(vec3(arrow_length, -self.line_width / 2.0, 0.0));{
                scad!(Cube(vec3(length - arrow_length, self.line_width, DOC_THICKNESS))),
            })
        } else if right_arrow {
            scad!(Translate(vec3(0.0, -self.line_width / 2.0, 0.0));{
                scad!(Cube(vec3(length - arrow_length, self.line_width, DOC_THICKNESS))),
            })
        } else {
            scad!(Translate(vec3(0.0, -self.line_width / 2.0, 0.0));{
                scad!(Cube(vec3(length, self.line_width, DOC_THICKNESS))),
            })
        };

        parent.add_child(line_obj);

        if left_arrow {
            parent.add_child(self.arrow(arrow_length, arrow_size));
        }

        if right_arrow {
            parent.add_child(scad!(Translate(vec3(length, 0.0, 0.0));{
                scad!(Rotate(180.0, vec3(0.0, 0.0, 1.0));{
                    self.arrow(arrow_length, arrow_size)
                })
            }));
        }

        parent
    }
}
