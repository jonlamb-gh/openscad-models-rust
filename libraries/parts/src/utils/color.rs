use scad::{scad, NamedColor, ScadObject};
use std::string::ToString;

/// Named colors
///
/// See https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#color
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Color {
    SandyBrown,
    SaddleBrown,
    PapayaWhip,
    Cornsilk,
    BlanchedAlmond,
    Bisque,
    BurlyWood,
    Sienna,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match *self {
            Color::SandyBrown => String::from("SandyBrown"),
            Color::SaddleBrown => String::from("SaddleBrown"),
            Color::PapayaWhip => String::from("PapayaWhip"),
            Color::Cornsilk => String::from("Cornsilk"),
            Color::BlanchedAlmond => String::from("BlanchedAlmond"),
            Color::Bisque => String::from("Bisque"),
            Color::BurlyWood => String::from("BurlyWood"),
            Color::Sienna => String::from("Sienna"),
        }
    }
}

impl Color {
    pub fn to_scad(&self) -> ScadObject {
        scad!(NamedColor(self.to_string()))
    }
}
