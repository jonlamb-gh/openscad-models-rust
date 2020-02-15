use crate::utils::{cm3_to_board_foot, BoardFoot, Centimeter};
use nalgebra::Vector3;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct BoardDimensions {
    /// size[0] is always with the grain
    /// size[1] is against the grain
    /// size[2] is the board thickness
    size: Vector3<Centimeter>,
}

impl BoardDimensions {
    pub fn new(length: Centimeter, width: Centimeter, thickness: Centimeter) -> Self {
        assert!(length > 0.0);
        assert!(width > 0.0);
        assert!(thickness > 0.0);
        Self {
            size: Vector3::new(length, width, thickness),
        }
    }

    pub fn size(&self) -> &Vector3<Centimeter> {
        &self.size
    }

    pub fn length(&self) -> Centimeter {
        self.size[0]
    }

    pub fn width(&self) -> Centimeter {
        self.size[1]
    }

    pub fn thickness(&self) -> Centimeter {
        self.size[2]
    }

    pub fn board_feet(&self) -> BoardFoot {
        cm3_to_board_foot(self.size)
    }
}

impl From<[Centimeter; 3]> for BoardDimensions {
    fn from(array: [Centimeter; 3]) -> Self {
        BoardDimensions::new(array[0], array[1], array[2])
    }
}
