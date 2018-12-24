use scad::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ObjectDescriptor {
    pub length: f32,
    pub width: f32,
    pub thickness: f32,
}

pub trait ObjectAssembler {
    fn describe(&self) -> ObjectDescriptor;

    fn has_color(&self) -> bool {
        self.object_color().is_some()
    }

    fn object_color(&self) -> Option<ScadObject> {
        None
    }

    // TODO - apply color here?
    fn assemble(&self) -> ScadObject;

    fn assemble_at(&self, x: f32, y: f32, z: f32) -> ScadObject {
        scad!(Translate(vec3(x, y, z));{
            self.assemble()
        })
    }

    fn assemble_center_xy(&self) -> ScadObject {
        let obj = self.describe();

        scad!(Translate(vec3(-obj.length / 2.0, -obj.width / 2.0, 0.0));{
            self.assemble()
        })
    }
}

pub fn some_color(color: Option<&'static str>) -> Option<String> {
    if let Some(c) = color {
        Some(c.to_string())
    } else {
        None
    }
}
