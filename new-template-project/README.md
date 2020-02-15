# template-project

## TODOs

- move things from here into the libs/etc
- add the other utils (`*`, `#`, `%`) to `ScadObject`, it already has `!` (important) exposed
- newtype or dimensioned derived for units? Centimeters/BoardFeet(foot?)/etc
- https://crates.io/crates/dimensioned
- try out https://github.com/openscad/openscad/issues/350
  * So that less `Union`'s are used in places where there are distinct parts

## NOTES

Redo the drawing/dimensions related trait(s).

the scad-assembler trait doesn't need to know about bounding regions/dimensions/etc

a Dimensions trait similar to BoardDimensions

