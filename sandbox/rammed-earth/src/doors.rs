use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Board;
use scad::*;

use config::*;

qstruct!(Doors(color: Option<&'static str>){
    outside_door: Board = Board::new(SINGLE_DOOR_MINOR, SINGLE_DOOR_MAJOR, DOOR_THICKNESS, color),
});

impl ObjectAssembler for Doors {
    // TODO - containing volume?
    fn describe(&self) -> ObjectDescriptor {
        self.outside_door.describe()
    }

    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.assemble_front_door(),
            self.assemble_bedroom_door(),
            self.assemble_back_doors(),
        })
    }
}

impl Doors {
    fn assemble_front_door(&self) -> ScadObject {
        scad!(Translate(vec3(ft_to_cm(18.0), ft_to_cm(34.0), DOOR_FRAME_THICKNESS));{
            self.assemble_xaligned(75.0)
        })
    }

    fn assemble_bedroom_door(&self) -> ScadObject {
        scad!(Translate(vec3(ft_to_cm(46.0) + SINGLE_DOOR_MAJOR, ft_to_cm(10.0), DOOR_FRAME_THICKNESS));{
            self.assemble_xaligned(-140.0)
        })
    }

    fn assemble_back_doors(&self) -> ScadObject {
        scad!(Union;{
            scad!(Translate(vec3(ft_to_cm(18.51), 0.0, DOOR_FRAME_THICKNESS));{
                self.assemble_xaligned(120.0)
            }),
            scad!(Translate(vec3(ft_to_cm(18.22) + DOUBLE_DOOR_MAJOR, -5.0, DOOR_FRAME_THICKNESS));{
                self.assemble_xaligned(60.0)
            }),
        })
    }

    fn assemble_xaligned(&self, angle: f32) -> ScadObject {
        scad!(Rotate(-90.0 - angle, z_axis());{
            scad!(Rotate(-90.0, y_axis());{
                self.outside_door.assemble()
            })
        })
    }
}
