use scad::*;

use super::ObjectAssembler;

//#[derive(Clone)]
pub struct DrawingParams {
    // title block ?
    show_frame: bool,
    doc_height: f32,
    // top_view params, pos/orient
}

impl Default for DrawingParams {
    fn default() -> DrawingParams {
        DrawingParams {
            show_frame: true,
            doc_height: 1.0,
        }
    }
}

pub trait DrawingBuilder: ObjectAssembler {
    fn describe_drawing(&self) -> DrawingParams;

    fn build_drawing(&self) -> ScadObject {
        let _params = self.describe_drawing();
        let obj = self.assemble();

        obj
    }
}

// build_frame()

fn build_top_view(obj: &ScadObject, params: &DrawingParams) -> ScadObject {
    obj
}
