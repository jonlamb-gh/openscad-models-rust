use crate::config::*;
use parts::prelude::*;
use scad::*;
use scad_assembler::ScadAssembler;

pub fn cut_45deg_ends(board: &Board) -> ScadObject {
    let d = board.dimensions();
    let cutout_len = d.width().get() * 2.0;
    let cutout = scad!(Translate(vec3(0.0, 0.0, -VISUAL_OVERRUN.get()));{
        scad!(Cube(vec3(
            cutout_len,
            d.width().get() * 2.0,
            (d.thickness() + (VISUAL_OVERRUN * 2.0)).get()
    )))});
    scad!(Difference;{
        board.assemble(),
        scad!(Rotate(45.0, vec3(0.0, 0.0, 1.0));{
            cutout.clone()
        }),
        scad!(Translate(vec3(d.length().get() + cutout_len, 0.0, 0.0));{
            scad!(Translate(vec3(-cutout_len, 0.0, 0.0));{
                scad!(Rotate(45.0, vec3(0.0, 0.0, 1.0));{
                    cutout
                })
            })
        })
    })
}
