use scad::*;

use super::{line, text, FONT_SCALE};

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
            do_circle: false,
            text: String::from("SDF"),
        }
    }
}

pub fn leader_line(params: &LeaderLineParams) -> ScadObject {
    let mut parent = scad!(Union);

    parent.add_child(scad!(
        Rotate(params.angle, vec3(0.0, 0.0, 1.0));{
            scad!(Translate(vec3(params.radius, 0.0, 0.0));{
                line(params.angle_length, true, false)
            })
        }
    ));

    parent.add_child(scad!(
        Rotate(params.angle, vec3(0.0, 0.0, 1.0));{
            scad!(Translate(vec3(params.radius + params.angle_length, 0.0, 0.0));{
                scad!(Rotate(-params.angle, vec3(0.0, 0.0, 1.0));{
                    match params.dir {
                        LeaderDirection::Left => left_leader(params),
                        LeaderDirection::Right => right_leader(params),
                    }
                })
            })
        }
    ));

    parent
}

fn left_leader(params: &LeaderLineParams) -> ScadObject {
    let text_len = params.text.len() as f32 * FONT_SCALE * 6.0;
    let space = FONT_SCALE * 6.0;

    let mut parent = scad!(Union);

    parent.add_child(scad!(
        Translate(vec3(-params.horiz_line_length, 0.0, 0.0));{
            line(params.horiz_line_length, false, false)
        }
    ));

    parent.add_child(scad!(
        Translate(vec3(-(params.horiz_line_length + space + text_len), -FONT_SCALE * 3.0, 0.0));{
            text(&params.text)
        }
    ));

    // TODO - call circle

    parent
}

fn right_leader(params: &LeaderLineParams) -> ScadObject {
    let space = FONT_SCALE * 6.0;

    let mut parent = scad!(Union);

    parent.add_child(line(params.horiz_line_length, false, false));

    parent.add_child(scad!(
        Translate(vec3(params.horiz_line_length + space, -FONT_SCALE * 3.0, 0.0));{
            text(&params.text)
        }
    ));

    // TODO - call circle

    parent
}
