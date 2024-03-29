use crate::utils::{BoardFoot, Centimeter, Foot, Inch};
use dim::ucum;
use scad::na::Vector3;
use std::fmt;

pub fn x_axis() -> Vector3<f32> {
    Vector3::new(1.0, 0.0, 0.0)
}

pub fn y_axis() -> Vector3<f32> {
    Vector3::new(0.0, 1.0, 0.0)
}

pub fn z_axis() -> Vector3<f32> {
    Vector3::new(0.0, 0.0, 1.0)
}

pub fn v3zero() -> Vector3<f32> {
    Vector3::new(0.0, 0.0, 0.0)
}

/// Convert international inch to centimeter
pub fn in_to_cm<U: Into<Inch>>(val: U) -> Centimeter {
    let m: ucum::Meter<f32> = val.into().as_f32() * ucum::f32consts::IN_I;
    (m / ucum::f32consts::CM).value_unsafe.into()
}

/// Convert centimeter to international inch
pub fn cm_to_in<U: Into<Centimeter>>(val: U) -> Inch {
    let m: ucum::Meter<f32> = val.into().as_f32() * ucum::f32consts::CM;
    (m / ucum::f32consts::IN_I).value_unsafe.into()
}

/// Convert international foot to centimeter
pub fn ft_to_cm<U: Into<Foot>>(val: U) -> Centimeter {
    let m: ucum::Meter<f32> = val.into().as_f32() * ucum::f32consts::FT_I;
    (m / ucum::f32consts::CM).value_unsafe.into()
}

/// Convert centimeter to international foot
pub fn cm_to_ft<U: Into<Centimeter>>(val: U) -> Foot {
    let m: ucum::Meter<f32> = val.into().as_f32() * ucum::f32consts::CM;
    (m / ucum::f32consts::FT_I).value_unsafe.into()
}

/// Convert inch to international foot
pub fn in_to_ft<U: Into<Inch>>(val: U) -> Foot {
    let m: ucum::Meter<f32> = val.into().as_f32() * ucum::f32consts::IN_I;
    (m / ucum::f32consts::FT_I).value_unsafe.into()
}

/// Convert length, width, thickness in centimeters to international board feet
/// https://en.wikipedia.org/wiki/Board_foot
pub fn cm3_to_board_foot<U: Into<Centimeter> + fmt::Debug + Copy + PartialEq + 'static>(
    dims: &Vector3<U>,
) -> BoardFoot {
    let volume_cin: ucum::Meter3<f32> = (cm_to_ft(dims.x).as_f32() * ucum::f32consts::FT_I)
        * (cm_to_in(dims.y).as_f32() * ucum::f32consts::IN_I)
        * (cm_to_in(dims.z).as_f32() * ucum::f32consts::IN_I);
    (volume_cin / ucum::f32consts::BF_I).value_unsafe.into()
}

/// Convert length, width, thickness in inches to international board feet
/// https://en.wikipedia.org/wiki/Board_foot
pub fn in3_to_board_foot<U: Into<Inch> + fmt::Debug + Copy + PartialEq + 'static>(
    dims: &Vector3<U>,
) -> BoardFoot {
    let volume_cin: ucum::Meter3<f32> = (in_to_ft(dims.x).as_f32() * ucum::f32consts::FT_I)
        * (dims.y.into().as_f32() * ucum::f32consts::IN_I)
        * (dims.z.into().as_f32() * ucum::f32consts::IN_I);
    (volume_cin / ucum::f32consts::BF_I).value_unsafe.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn in_to_cm_conversions() {
        assert_relative_eq!(in_to_cm(0.0), 0.0);
        assert_relative_eq!(in_to_cm(0.5), 1.27);
        assert_relative_eq!(in_to_cm(1.0), 2.54);
        assert_relative_eq!(in_to_cm(2.0), 5.08);

        assert_relative_eq!(cm_to_in(0.0), 0.0);
        assert_relative_eq!(cm_to_in(1.27), 0.5);
        assert_relative_eq!(cm_to_in(2.54), 1.0);
        assert_relative_eq!(cm_to_in(5.08), 2.0);
    }

    #[test]
    fn ft_to_cm_conversions() {
        assert_relative_eq!(ft_to_cm(0.0), 0.0);
        assert_relative_eq!(ft_to_cm(1.0 / 12.0), 2.54);
        assert_relative_eq!(ft_to_cm(0.5), 15.24);
        assert_relative_eq!(ft_to_cm(1.0), 30.48);
        assert_relative_eq!(ft_to_cm(2.0), 60.96);

        assert_relative_eq!(cm_to_ft(0.0), 0.0);
        assert_relative_eq!(cm_to_ft(2.54), 1.0 / 12.0);
        assert_relative_eq!(cm_to_ft(15.24), 0.5);
        assert_relative_eq!(cm_to_ft(30.48), 1.0);
        assert_relative_eq!(cm_to_ft(60.96), 2.0);
    }

    #[test]
    fn board_foot_conversions() {
        let dims = [30.48, 30.48, 2.54];
        assert_relative_eq!(cm3_to_board_foot(&dims.into()), 1.0);
        let dims = [ft_to_cm(1.0), ft_to_cm(1.0), in_to_cm(1.0)];
        assert_relative_eq!(cm3_to_board_foot(&dims.into()), 1.0);
        let dims = [in_to_cm(12.0), in_to_cm(12.0), in_to_cm(1.0)];
        assert_relative_eq!(cm3_to_board_foot(&dims.into()), 1.0);
    }
}
