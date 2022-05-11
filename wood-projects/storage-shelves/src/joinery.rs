use crate::config::*;
use parts::prelude::*;
use scad::*;

pub fn cut_half_shelf_med_board_screw_holes(obj: ScadObject) -> ScadObject {
    let screw_cutouts = shelf_board_screw_pair();

    let mut root = ScadObject::new(ScadElement::Difference);
    root.add_child(obj);

    for n in 0..NUM_SHORT_BOARDS_PER_HALF_SHELF {
        let offset = HALF_SHELF_SUPPORT_BOARD_OFFSET * Inch::new(n as _);
        root.add_child(scad!(Translate(vec3(offset.get(), 0.0, 0.0));{
            screw_cutouts.clone()
        }));
    }

    root
}

pub fn cut_full_shelf_long_board_screw_holes(obj: ScadObject) -> ScadObject {
    let screw_cutouts = shelf_board_screw_pair();

    let mut root = ScadObject::new(ScadElement::Difference);
    root.add_child(obj);

    for n in 0..NUM_SHORT_BOARDS_PER_FULL_SHELF {
        let offset = FULL_SHELF_SUPPORT_BOARD_OFFSET * Inch::new(n as _);
        root.add_child(scad!(Translate(vec3(offset.get(), 0.0, 0.0));{
            screw_cutouts.clone()
        }));
    }

    root
}

pub fn cut_vert_support_long_board_screw_holes_a(obj: ScadObject) -> ScadObject {
    let screw_cutouts = vert_support_screw_pair();

    let mut root = ScadObject::new(ScadElement::Difference);
    root.add_child(obj);

    for offset in vertical_shelf_accum_heights().into_iter().skip(1) {
        root.add_child(scad!(Translate(vec3(offset.get(), 0.0, 0.0));{
            screw_cutouts.clone()
        }));
    }

    root
}

pub fn cut_vert_support_long_board_screw_holes_b(obj: ScadObject) -> ScadObject {
    let screw_cutouts = vert_support_screw_pair();

    let offsets = vertical_shelf_accum_heights();

    let mut root = ScadObject::new(ScadElement::Difference);
    root.add_child(obj);

    root.add_child(scad!(Union;{
        scad!(Translate(vec3((offsets[1] - BOARD_WIDTH).get(), 0.0, 0.0));{
            screw_cutouts.clone()
        }),
        scad!(Translate(vec3((offsets[1] - BOARD_WIDTH).get(), -(BOARD_WIDTH / 2.0).get(), 0.0));{
            screw_cutouts.clone()
        })
    }));

    root.add_child(
        scad!(Translate(vec3((offsets[2] - BOARD_WIDTH).get(), -(BOARD_WIDTH / 2.0).get(), 0.0));{
            screw_cutouts
        }),
    );

    let screw_cutouts = shelf_board_screw_pair();

    root.add_child(
        scad!(Translate(vec3((offsets[3] + (BOARD_WIDTH / 4.0)).get(), 0.0, 0.0));{
            screw_cutouts.clone()
        }),
    );

    root.add_child(
        scad!(Translate(vec3((offsets[4] + (BOARD_WIDTH / 4.0)).get(), 0.0, 0.0));{
            screw_cutouts
        }),
    );

    root
}

pub fn cut_vert_support_long_board_screw_holes_c(obj: ScadObject) -> ScadObject {
    let screw_cutouts = vert_support_screw_pair();

    let offsets = vertical_shelf_accum_heights();

    let mut root = ScadObject::new(ScadElement::Difference);
    root.add_child(obj);

    root.add_child(
        scad!(Translate(vec3((offsets[0] - BOARD_WIDTH).get(), 0.0, 0.0));{
            screw_cutouts.clone()
        }),
    );

    root.add_child(
        scad!(Translate(vec3((offsets[1] - BOARD_WIDTH).get(), -(BOARD_WIDTH / 2.0).get(), 0.0));{
            screw_cutouts
        }),
    );

    let screw_cutouts = shelf_board_screw_pair();

    root.add_child(
        scad!(Translate(vec3((offsets[3] + (BOARD_WIDTH / 4.0)).get(), 0.0, 0.0));{
            screw_cutouts.clone()
        }),
    );

    root.add_child(
        scad!(Translate(vec3((offsets[4] + (BOARD_WIDTH / 4.0)).get(), 0.0, 0.0));{
            screw_cutouts
        }),
    );

    root
}

pub fn cut_vert_support_long_board_screw_holes_d(obj: ScadObject) -> ScadObject {
    let screw_cutouts = vert_support_screw_pair();

    let mut root = ScadObject::new(ScadElement::Difference);
    root.add_child(obj);

    let yo = BOARD_WIDTH / 2.0;
    for offset in vertical_shelf_accum_heights()
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| (i != 1) && (i != 2))
        .map(|(_, o)| o)
    {
        root.add_child(scad!(Translate(vec3(offset.get(), -yo.get(), 0.0));{
            screw_cutouts.clone()
        }));
    }

    root
}

fn screw_hole_cutout() -> ScadObject {
    scad!(Cylinder(
        BOARD_THICKNESS.get(),
        CircleType::Diameter(SCREW_HOLE_DIAMETER.get())
    ))
}

fn shelf_board_screw_pair() -> ScadObject {
    let cutout = screw_hole_cutout();
    let xo = BOARD_THICKNESS / 2.0;
    let yo = BOARD_WIDTH / 4.0;
    scad!(Union;{
        scad!(Translate(vec3(xo.get(), yo.get(), 0.0));{
            cutout.clone()
        }),
        scad!(Translate(vec3(xo.get(), (BOARD_WIDTH - yo).get(), 0.0));{
            cutout
        }),
    })
}

fn vert_support_screw_pair() -> ScadObject {
    let cutout = screw_hole_cutout();
    let xo = BOARD_WIDTH / 4.0;
    let yo = BOARD_WIDTH - (BOARD_THICKNESS / 2.0);
    scad!(Union;{
        scad!(Translate(vec3(xo.get(), yo.get(), 0.0));{
            cutout.clone()
        }),
        scad!(Translate(vec3((BOARD_WIDTH - xo).get(), yo.get(), 0.0));{
            cutout
        }),
    })
}
