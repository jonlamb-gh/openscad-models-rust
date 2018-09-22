use scad::*;

qstruct!(Drawing(doc_scale: f32){
    line_width: f32 = 0.025 * doc_scale,
    spacing: f32 = 0.1 * doc_scale,
    font_scale: f32 = (0.025 * doc_scale) * 0.7,
});
