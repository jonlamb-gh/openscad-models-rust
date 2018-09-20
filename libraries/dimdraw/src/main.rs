// TODO - move this into docs or examples and restructure dirs

extern crate scad;

mod lib;

use lib::*;
use scad::*;

fn main() {
    let mut sfile = ScadFile::new();

    sfile.set_detail(50);

    let mut object = scad!(Color(vec3(0.0, 0.0, 0.0)));

    object.add_child(line(10.0, true, true));

    object.add_child(scad!(Translate(vec3(0.0, 0.5, 0.0));{
        line(10.0, false, true)
    }));

    object.add_child(scad!(Translate(vec3(0.0, 1.0, 0.0));{
        line(10.0, true, false)
    }));

    object.add_child(scad!(Translate(vec3(0.0, 1.5, 0.0));{
        line(10.0, false, false)
    }));

    object.add_child(scad!(Translate(vec3(1.0, 2.0, 0.0));{
        text("A String")
    }));

    object.add_child(scad!(Translate(vec3(0.0, 2.5, 0.0));{
        dim_line(5.0, DimLocation::Center)
    }));

    object.add_child(scad!(Translate(vec3(0.0, 3.0, 0.0));{
        dim_line(5.0, DimLocation::Left)
    }));

    object.add_child(scad!(Translate(vec3(0.0, 3.5, 0.0));{
        dim_line(5.0, DimLocation::Right)
    }));

    object.add_child(scad!(Translate(vec3(0.0, 4.0, 0.0));{
        dim_line(5.0, DimLocation::Outside)
    }));

    sfile.add_object(object);

    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    println!("{}", filename);
}
