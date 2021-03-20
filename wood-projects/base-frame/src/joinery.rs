use crate::config::*;
use parts::prelude::*;
use scad::*;
use static_assertions::const_assert;

/// Assumes the other board has equal width/thickness
pub fn cut_bottom_ends(dims: &BoardDimensions, obj: ScadObject) -> ScadObject {
    let s = dims.unitless_size();
    let cutout_size = vec3(s.y, s.y * 2.0, s.z / 2.0);
    let cutout = scad!(Cube(cutout_size));
    scad!(Difference;{
        obj,
        cutout.clone(),
        scad!(Translate(vec3(s.x - s.y, 0.0, 0.0));{
            cutout
        })
    })
}

/// Assumes the other board has equal width/thickness
pub fn cut_top_ends(dims: &BoardDimensions, obj: ScadObject) -> ScadObject {
    let s = dims.unitless_size();
    let cutout_size = vec3(s.y, s.y * 2.0, s.z / 2.0);
    let cutout = scad!(Translate(vec3(0.0, 0.0, s.z / 2.0));{
        scad!(Cube(cutout_size))
    });
    scad!(Difference;{
        obj,
        cutout.clone(),
        scad!(Translate(vec3(s.x - s.y, 0.0, 0.0));{
            cutout
        })
    })
}

pub fn cut_slat_board_slots(dims: &BoardDimensions, obj: ScadObject) -> ScadObject {
    let s = dims.unitless_size();
    let cutout_size = vec3(SLAT_BOARD_WIDTH.get(), s.y, SLAT_BOARD_THICKNESS.get());
    let cutout = scad!(Translate(vec3(0.0, 0.0, s.z - SLAT_BOARD_THICKNESS.get()));{
        scad!(Cube(cutout_size))
    });
    let start = dims.length() / 2.0;
    let slat_offsets = slat_offsets();
    let mut root = ScadObject::new(ScadElement::Difference);
    root.add_child(obj);
    for rel_offset in slat_offsets.into_iter() {
        let offset_a = start + rel_offset;
        let offset_b = start - SLAT_BOARD_WIDTH - rel_offset;
        root.add_child(scad!(Translate(vec3(offset_a.get(), 0.0, 0.0));{
            cutout.clone()
        }));
        root.add_child(scad!(Translate(vec3(offset_b.get(), 0.0, 0.0));{
            cutout.clone()
        }));
    }
    root
}

pub fn cut_bolt_holes(dims: &BoardDimensions, obj: ScadObject) -> ScadObject {
    let s = dims.unitless_size();
    let cutout = scad!(Translate(vec3(0.0, s.y, s.z / 2.0));{
        scad!(Rotate(90.0, vec3(1.0, 0.0, 0.0));{
            scad!(Cylinder(s.y, CircleType::Diameter(BOLT_HOLE_DIAMETER.get())))
        })
    });
    scad!(Difference;{
        obj,
        scad!(Translate(vec3((s.x / 2.0) - SHORT_FRAME_BOLT_OFFSET.get(), 0.0, 0.0));{
            cutout.clone()
        }),
        scad!(Translate(vec3((s.x / 2.0) + SHORT_FRAME_BOLT_OFFSET.get(), 0.0, 0.0));{
            cutout
        })
    })
}

const_assert!(NUM_SLAT_BOARDS % 2 == 0);
pub fn slat_offsets() -> Vec<Centimeter> {
    let start = SLAT_BOARD_SEP_DISTANCE / 2.0;
    let mut offsets = Vec::new();
    for idx in 0..(NUM_SLAT_BOARDS / 2) {
        let offset =
            start + Centimeter::new(idx as f32) * (SLAT_BOARD_WIDTH + SLAT_BOARD_SEP_DISTANCE);
        assert!(
            offset < (LONG_FRAME_BOARD_LENGTH / 2.0),
            "Slat board offset {} would overrun the frame",
            offset
        );
        offsets.push(offset);
    }
    offsets
}
