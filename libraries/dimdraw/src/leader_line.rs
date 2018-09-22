use scad::*;

use super::DOC_THICKNESS;
use drawing::Drawing;

pub enum LeaderDirection {
    Left,
    Right,
}

pub struct LeaderLineParams {
    pub angle: f32,
    pub radius: f32,
    pub angle_length: f32,
    pub horiz_line_length: f32,
    pub dir: LeaderDirection,
    pub do_circle: bool,
    pub text: String,
}

impl Default for LeaderLineParams {
    fn default() -> LeaderLineParams {
        LeaderLineParams {
            angle: 45.0,
            radius: 1.0,
            angle_length: 1.0,
            horiz_line_length: 1.0,
            dir: LeaderDirection::Right,
            do_circle: true,
            text: String::new(),
        }
    }
}

impl Drawing {
    pub fn leader_line(&self, params: &LeaderLineParams) -> ScadObject {
        let mut parent = scad!(Union);

        parent.add_child(scad!(
            Rotate(params.angle, vec3(0.0, 0.0, 1.0));{
                scad!(Translate(vec3(params.radius, 0.0, 0.0));{
                    self.line(params.angle_length, true, false)
                })
            }
        ));

        parent.add_child(scad!(
            Rotate(params.angle, vec3(0.0, 0.0, 1.0));{
                scad!(Translate(vec3(params.radius + params.angle_length, 0.0, 0.0));{
                    scad!(Rotate(-params.angle, vec3(0.0, 0.0, 1.0));{
                        match params.dir {
                            LeaderDirection::Left => self.left_leader(params),
                            LeaderDirection::Right => self.right_leader(params),
                        }
                    })
                })
            }
        ));

        parent
    }

    fn left_leader(&self, params: &LeaderLineParams) -> ScadObject {
        let text_len = params.text.len() as f32 * self.font_scale * 6.0;
        let space = self.font_scale * 6.0;

        let mut parent = scad!(Union);

        parent.add_child(scad!(
            Translate(vec3(-params.horiz_line_length, 0.0, 0.0));{
                self.line(params.horiz_line_length, false, false)
            }
        ));

        parent.add_child(scad!(
            Translate(vec3(-(params.horiz_line_length + space + text_len), -self.font_scale * 3.0, 0.0));{
                self.text(&params.text)
            }
        ));

        if params.do_circle {
            parent.add_child(self.circle(params));
        }

        parent
    }

    fn right_leader(&self, params: &LeaderLineParams) -> ScadObject {
        let space = self.font_scale * 6.0;

        let mut parent = scad!(Union);

        parent.add_child(self.line(params.horiz_line_length, false, false));

        parent.add_child(scad!(
            Translate(vec3(params.horiz_line_length + space, -self.font_scale * 3.0, 0.0));{
                self.text(&params.text)
            }
        ));

        if params.do_circle {
            parent.add_child(self.circle(params));
        }

        parent
    }

    fn circle(&self, params: &LeaderLineParams) -> ScadObject {
        let text_len = params.text.len() as f32 * self.font_scale * 6.0;
        let space = self.font_scale * 6.0;

        let pos_x = match params.dir {
            LeaderDirection::Left => -(params.horiz_line_length + space + (text_len / 2.0)),
            LeaderDirection::Right => params.horiz_line_length + space + (text_len / 2.0),
        };

        scad!(Translate(vec3(pos_x, 0.0, 0.0));{
            scad!(Difference;{
                scad!(Cylinder(DOC_THICKNESS, CircleType::Radius(text_len + space - self.line_width))),
                scad!(Translate(vec3(0.0, 0.0, -0.01));{
                    scad!(Cylinder(
                        DOC_THICKNESS + 0.02,
                        CircleType::Radius(text_len + space - (self.line_width * 2.0)))
                    )
                })
            })
        })
    }
}
