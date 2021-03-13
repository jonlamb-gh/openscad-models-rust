use scad::ScadObject;

// TODO - rename/new trait ScadObjectExt, returns ScadObject

pub trait ScadAssembler {
    /// Assemble into a scad element with zero or more children
    fn assemble(&self) -> ScadObject;
}
