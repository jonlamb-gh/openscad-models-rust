use scad::*;

use super::{line, text, FONT_SCALE};

pub enum DimLocation {
    Center,
    Left,
    Right,
    Outside,
}

pub fn dim_line(length: f32, loc: DimLocation) -> ScadObject {
    let len_str = length.to_string();
    let space = len_str.len() as f32 * FONT_SCALE * 7.0;

    match loc {
        DimLocation::Center => center_dim(length, space, &len_str),
        DimLocation::Left => left_dim(length, space, &len_str),
        DimLocation::Right => right_dim(length, space, &len_str),
        DimLocation::Outside => outside_dim(length, space, &len_str),
    }
}

fn center_dim(length: f32, space: f32, len_str: &str) -> ScadObject {
    let mut parent = scad!(Union);

    parent.add_child(line((length / 2.0) - (space / 2.0), true, false));
    parent.add_child(scad!(
        Translate(vec3((length / 2.0) - (space / 2.0) * 0.8, -FONT_SCALE * 3.0, 0.0));{
            text(&len_str)
        }
    ));
    parent.add_child(scad!(
        Translate(vec3((length / 2.0) + (space / 2.0), 0.0, 0.0));{
            line((length / 2.0) - (space / 2.0), false, true)
        }
    ));

    parent
}

fn left_dim(length: f32, space: f32, len_str: &str) -> ScadObject {
    let mut parent = scad!(Union);

    parent.add_child(line(length, true, true));
    parent.add_child(scad!(
        Translate(vec3(-space, -FONT_SCALE * 3.0, 0.0));{
            text(&len_str)
        }
    ));

    parent
}

fn right_dim(length: f32, space: f32, len_str: &str) -> ScadObject {
    let mut parent = scad!(Union);

    parent.add_child(line(length, true, true));
    parent.add_child(scad!(
        Translate(vec3(length + space, -FONT_SCALE * 3.0, 0.0));{
            text(&len_str)
        }
    ));

    parent
}

fn outside_dim(length: f32, space: f32, len_str: &str) -> ScadObject {
    let mut parent = scad!(Union);

    parent.add_child(scad!(
        Rotate(180.0, vec3(0.0, 1.0, 0.0));{
            line(length / 2.0, true, false)
        }
    ));
    parent.add_child(scad!(
        Translate(vec3((length / 2.0) - (space / 2.0) * 0.8, -FONT_SCALE * 3.0, 0.0));{
            text(&len_str)
        }
    ));
    parent.add_child(scad!(
        Translate(vec3(length, 0.0, 0.0));{
            line(length / 2.0, true, false)
        }
    ));

    parent
}
