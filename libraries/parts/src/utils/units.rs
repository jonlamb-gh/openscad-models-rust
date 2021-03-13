use crate::{
    from_newtype, newtype_add, newtype_approx_impls, newtype_deref, newtype_deref_mut,
    newtype_display, newtype_div, newtype_from, newtype_helper_impls, newtype_mul, newtype_sub,
};
use std::fmt;
use std::ops::{Deref, DerefMut};

pub trait Unit:
    Deref<Target = f32>
    + DerefMut<Target = f32>
    + From<f32>
    + Into<f32>
    + Copy
    + PartialEq
    + fmt::Debug
    + private::Sealed
{
    const UNIT_NAME: &'static str;

    fn get(&self) -> f32 {
        *self.deref()
    }

    // TODO - conversions
    // as_cm()
    // as_ft
    // ...
}

pub trait BoardFootExt {
    fn board_feet(&self) -> BoardFoot;
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
#[repr(transparent)]
pub struct BoardFoot(f32);
newtype_display!(BoardFoot);
newtype_from!(BoardFoot, f32);
from_newtype!(BoardFoot, f32);
newtype_deref!(BoardFoot, f32);
newtype_deref_mut!(BoardFoot, f32);
newtype_mul!(BoardFoot);
newtype_div!(BoardFoot);
newtype_add!(BoardFoot);
newtype_sub!(BoardFoot);
newtype_helper_impls!(BoardFoot);
newtype_approx_impls!(BoardFoot);
impl private::Sealed for BoardFoot {}
impl Unit for BoardFoot {
    const UNIT_NAME: &'static str = "BoardFoot";
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
#[repr(transparent)]
pub struct Foot(f32);
newtype_display!(Foot);
newtype_from!(Foot, f32);
from_newtype!(Foot, f32);
newtype_deref!(Foot, f32);
newtype_deref_mut!(Foot, f32);
newtype_mul!(Foot);
newtype_div!(Foot);
newtype_add!(Foot);
newtype_sub!(Foot);
newtype_helper_impls!(Foot);
newtype_approx_impls!(Foot);
impl private::Sealed for Foot {}
impl Unit for Foot {
    const UNIT_NAME: &'static str = "Foot";
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
#[repr(transparent)]
pub struct Inch(f32);
newtype_display!(Inch);
newtype_from!(Inch, f32);
from_newtype!(Inch, f32);
newtype_deref!(Inch, f32);
newtype_deref_mut!(Inch, f32);
newtype_mul!(Inch);
newtype_div!(Inch);
newtype_add!(Inch);
newtype_sub!(Inch);
newtype_helper_impls!(Inch);
newtype_approx_impls!(Inch);
impl private::Sealed for Inch {}
impl Unit for Inch {
    const UNIT_NAME: &'static str = "Inch";
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
#[repr(transparent)]
pub struct Meter(f32);
newtype_display!(Meter);
newtype_from!(Meter, f32);
from_newtype!(Meter, f32);
newtype_deref!(Meter, f32);
newtype_deref_mut!(Meter, f32);
newtype_mul!(Meter);
newtype_div!(Meter);
newtype_add!(Meter);
newtype_sub!(Meter);
newtype_helper_impls!(Meter);
newtype_approx_impls!(Meter);
impl private::Sealed for Meter {}
impl Unit for Meter {
    const UNIT_NAME: &'static str = "Meter";
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
#[repr(transparent)]
pub struct Centimeter(f32);
newtype_display!(Centimeter);
newtype_from!(Centimeter, f32);
from_newtype!(Centimeter, f32);
newtype_deref!(Centimeter, f32);
newtype_deref_mut!(Centimeter, f32);
newtype_mul!(Centimeter);
newtype_div!(Centimeter);
newtype_add!(Centimeter);
newtype_sub!(Centimeter);
newtype_helper_impls!(Centimeter);
newtype_approx_impls!(Centimeter);
impl private::Sealed for Centimeter {}
impl Unit for Centimeter {
    const UNIT_NAME: &'static str = "Centimeter";
}

mod private {
    pub trait Sealed {}
}
