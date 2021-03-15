use crate::utils::Color;
use scad::{ScadElement, ScadObject};

pub trait ScadAssembler {
    /// Assemble into a scad element with zero or more children
    fn assemble(&self) -> ScadObject;

    fn color(&self) -> Option<Color> {
        None
    }

    fn assemble_with<F>(&self, f: F) -> ScadObject
    where
        F: Fn(ScadObject, Option<Color>) -> ScadObject,
    {
        let obj = self.assemble();
        let maybe_color = self.color();
        f(obj, maybe_color)
    }
}

/// Uses an explicit render() or color()->render() as the root element as a workaround
/// for the preview rendering artifacts
/// https://github.com/openscad/openscad/issues/1000
pub fn color_or_render_root(obj: ScadObject, color: Option<Color>) -> ScadObject {
    let render = ScadObject::new(ScadElement::Render);
    let mut root = if let Some(c) = color {
        let mut root = c.to_scad();
        root.add_child(render);
        root
    } else {
        render
    };
    root.add_child(obj);
    root
}
