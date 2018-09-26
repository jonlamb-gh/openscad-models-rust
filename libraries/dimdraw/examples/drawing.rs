extern crate dimdraw;
extern crate scad;

use dimdraw::*;
use scad::*;

qstruct!(Part() {
    length: f32 = 10.0,
    width: f32 = 8.0,
    thickness: f32 = 6.0,
});

impl ObjectAssembler for Part {
    fn assemble(&self) -> ScadObject {
        if let Some(mut c) = self.object_color() {
            c.add_child(scad!(Cube(vec3(self.length, self.width, self.thickness))));
            c
        } else {
            scad!(Cube(vec3(self.length, self.width, self.thickness)))
        }
    }

    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: self.length,
            width: self.width,
            thickness: self.thickness,
        }
    }

    fn object_color(&self) -> Option<ScadObject> {
        Some(scad!(NamedColor("BurlyWood".to_string())))
    }
}

impl DrawingAssembler for Part {
    fn drawing_params(&self) -> DrawingParams {
        DrawingParams::default()
    }
}

fn main() {
    let mut sfile = ScadFile::new();

    sfile.set_detail(100);

    let part = Part::new();

    let object = part.assemble_drawing();

    sfile.add_object(object);

    let filename = &format!("{}.scad", env!("CARGO_PKG_NAME"));
    let result = sfile.write_to_file(filename.to_string());
    assert_eq!(result, true);

    println!("{}", filename);
}
