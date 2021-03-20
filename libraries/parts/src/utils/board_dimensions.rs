use crate::utils::{cm3_to_board_foot, BoardFoot, BoardFootExt, Centimeter, Unit};
use scad::na::Vector3;
use std::fmt;

/// Default unit is centimeter
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct BoardDimensions<U: Unit + 'static = Centimeter> {
    /// size[0] is with the grain
    /// size[1] is against the grain
    /// size[2] is the board thickness
    size: Vector3<U>,
}

impl<U> BoardDimensions<U>
where
    U: Unit + 'static,
{
    pub fn new(length: U, width: U, thickness: U) -> Self {
        assert!(length.get() > 0.0);
        assert!(width.get() > 0.0);
        assert!(thickness.get() > 0.0);
        Self {
            size: Vector3::new(length, width, thickness),
        }
    }

    pub fn units(&self) -> &'static str {
        U::UNIT_NAME
    }

    pub fn size(&self) -> &Vector3<U> {
        &self.size
    }

    pub fn unitless_size(&self) -> Vector3<f32> {
        Vector3::new(
            self.size[0].into(),
            self.size[1].into(),
            self.size[2].into(),
        )
    }

    pub fn length(&self) -> U {
        self.size[0]
    }

    pub fn width(&self) -> U {
        self.size[1]
    }

    pub fn thickness(&self) -> U {
        self.size[2]
    }
}

impl<U> fmt::Display for BoardDimensions<U>
where
    U: Unit + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} x {} x {}",
            self.length(),
            self.width(),
            self.thickness(),
        )
    }
}

impl<U> BoardFootExt for BoardDimensions<U>
where
    U: Unit + 'static + Into<Centimeter>,
{
    fn board_feet(&self) -> BoardFoot {
        cm3_to_board_foot(&self.size)
    }
}

impl<U> From<[U; 3]> for BoardDimensions<U>
where
    U: Unit + 'static,
{
    fn from(array: [U; 3]) -> Self {
        BoardDimensions::new(array[0], array[1], array[2])
    }
}

impl<U> From<(U, U, U)> for BoardDimensions<U>
where
    U: Unit + 'static,
{
    fn from(tuple: (U, U, U)) -> Self {
        BoardDimensions::new(tuple.0, tuple.1, tuple.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn board_foot_ext() {
        let size: [Centimeter; 3] = [30.48.into(), 30.48.into(), 2.54.into()];
        let dims = BoardDimensions::from(size);
        assert_relative_eq!(dims.board_feet(), 1.0);
    }
}
