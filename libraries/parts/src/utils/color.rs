use scad::{scad, NamedColor, ScadObject};
use std::string::ToString;

/// Named colors
///
/// See https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#color
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Color {
    SandyBrown,
    PapayaWhip,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match *self {
            Color::SandyBrown => String::from("SandyBrown"),
            Color::PapayaWhip => String::from("PapayaWhip"),
        }
    }
}

impl Color {
    pub fn to_scad(&self) -> ScadObject {
        scad!(NamedColor(self.to_string()))
    }
}
