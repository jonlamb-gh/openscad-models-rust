use scad::*;

use drawing::Drawing;

pub enum DimLocation {
    Center,
    Left,
    Right,
    Outside,
}

impl Drawing {
    pub fn dim_line(&self, length: f32, loc: DimLocation) -> ScadObject {
        let space = length.to_string().len() as f32 * self.font_scale * 7.0;

        match loc {
            DimLocation::Center => self.center_dim(length, space),
            DimLocation::Left => self.left_dim(length, space),
            DimLocation::Right => self.right_dim(length, space),
            DimLocation::Outside => self.outside_dim(length, space),
        }
    }

    fn center_dim(&self, length: f32, space: f32) -> ScadObject {
        let mut parent = scad!(Union);

        parent.add_child(self.line((length / 2.0) - (space / 2.0), true, false));
        parent.add_child(scad!(
            Translate(vec3((length / 2.0) - (space / 2.0) * 0.8, -self.font_scale * 3.0, 0.0));{
                self.text(&length.to_string())
            }
        ));
        parent.add_child(scad!(
            Translate(vec3((length / 2.0) + (space / 2.0), 0.0, 0.0));{
                self.line((length / 2.0) - (space / 2.0), false, true)
            }
        ));

        parent
    }

    fn left_dim(&self, length: f32, space: f32) -> ScadObject {
        let mut parent = scad!(Union);

        parent.add_child(self.line(length, true, true));
        parent.add_child(scad!(
            Translate(vec3(-space, -self.font_scale * 3.0, 0.0));{
                self.text(&length.to_string())
            }
        ));

        parent
    }

    fn right_dim(&self, length: f32, space: f32) -> ScadObject {
        let mut parent = scad!(Union);

        parent.add_child(self.line(length, true, true));
        parent.add_child(scad!(
            Translate(vec3(length + space, -self.font_scale * 3.0, 0.0));{
                self.text(&length.to_string())
            }
        ));

        parent
    }

    fn outside_dim(&self, length: f32, space: f32) -> ScadObject {
        let mut parent = scad!(Union);

        parent.add_child(scad!(
            Rotate(180.0, vec3(0.0, 1.0, 0.0));{
                self.line(length / 2.0, true, false)
            }
        ));
        parent.add_child(scad!(
            Translate(vec3((length / 2.0) - (space / 2.0) * 0.8, -self.font_scale * 3.0, 0.0));{
                self.text(&length.to_string())
            }
        ));
        parent.add_child(scad!(
            Translate(vec3(length, 0.0, 0.0));{
                self.line(length / 2.0, true, false)
            }
        ));

        parent
    }
}
