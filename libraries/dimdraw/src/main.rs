// TODO - move this into docs or examples and restructure dirs

extern crate scad;

mod lib;

use lib::*;
use scad::*;

qstruct!(Part() {
    length: f32 = 10.0,
    width: f32 = 8.0,
    thickness: f32 = 6.0,
});

impl ObjectAssembler for Part {
    fn assemble(&self) -> ScadObject {
        scad!(Cube(vec3(self.length, self.width, self.thickness,)))
    }
}

impl DrawingAssembler for Part {
    fn describe_drawing(&self) -> DrawingParams {
        DrawingParams::default()
    }

    fn describe_object(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: self.length,
            width: self.width,
            thickness: self.thickness,
        }
    }
}

fn main() {
    let mut sfile = ScadFile::new();

    sfile.set_detail(100);

    let part = Part::new();

    let object = part.assemble_drawing();

    /*
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

    object.add_child(scad!(Translate(vec3(0.0, 4.5, 0.0));{
        leader_line(&LeaderLineParams::default())
    }));
    */

    sfile.add_object(object);

    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    println!("{}", filename);
}
