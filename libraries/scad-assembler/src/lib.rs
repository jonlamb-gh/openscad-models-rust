use scad::ScadObject;

pub trait ScadAssembler {
    // TODO - reference frame is relative
    /// Assemble into a scad element with zero or more children
    fn assemble(&self) -> ScadObject;

    // TODO - not sure this makes sense here?
    // Returns the color (if any) as a ScadObject (ie NamedColor or Color)
    //fn color(&self) -> Option<ScadObject> {
    //    None
    //}
}
