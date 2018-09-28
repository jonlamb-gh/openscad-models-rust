use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Wall;
use scad::*;

use config::*;

qstruct!(OuterWalls(color: Option<&'static str>) {
    l4: Wall = Wall::new(
        OUTER_WALL_L4_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color),
    l6: Wall = Wall::new(
        OUTER_WALL_L6_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color),
    l8: Wall = Wall::new(
        OUTER_WALL_L8_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color),
    l10: Wall = Wall::new(
        OUTER_WALL_L10_LENGTH,
        OUTER_WALL_WIDTH,
        OUTER_WALL_THICKNESS,
        color),
});

impl OuterWalls {
    fn assemble_rows(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // row A
        parent.add_child(self.l10.assemble_xaligned());
        let x_offset = ft_to_cm(18.0);
        parent.add_child(scad!(Translate(vec3(x_offset, 0.0, 0.0));{
                self.l8.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(26.0);
        parent.add_child(scad!(Translate(vec3(x_offset, 0.0, 0.0));{
                self.l10.assemble_xaligned()
            }));

        // row B
        let y_offset = ft_to_cm(8.0);
        let x_offset = ft_to_cm(34.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8.assemble_xaligned()
        }));
        let x_offset = ft_to_cm(42.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8.assemble_xaligned()
        }));

        // row E
        let y_offset = ft_to_cm(36.0) - self.l4.thickness();
        let x_offset = 0.0;
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l10.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(18.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l6.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(24.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l6.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(30.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
                self.l10.assemble_xaligned()
            }));
        let x_offset = ft_to_cm(40.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10.assemble_xaligned()
        }));

        parent
    }

    fn assemble_columns(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // col 1
        let x_offset = 0.0;
        let y_offset = self.l8.thickness();
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l6.assemble_yaligned()
        }));
        let y_offset = self.l8.thickness() + ft_to_cm(6.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10.assemble_yaligned()
        }));
        let y_offset = self.l8.thickness() + ft_to_cm(16.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10.assemble_yaligned()
        }));
        let y_offset = self.l8.thickness() + ft_to_cm(26.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l6.assemble_yaligned()
        }));

        // col 5
        let x_offset = ft_to_cm(36.0) - self.l8.thickness();
        let y_offset = self.l8.thickness();
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l6.assemble_yaligned()
        }));

        // col 7
        let x_offset = ft_to_cm(50.0);
        let y_offset = ft_to_cm(8.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10.assemble_yaligned()
        }));
        let y_offset = ft_to_cm(18.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8.assemble_yaligned()
        }));
        let y_offset = ft_to_cm(26.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10.assemble_yaligned()
        }));

        parent
    }
}

impl ObjectAssembler for OuterWalls {
    // TODO - containing volume?
    fn describe(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            length: ft_to_cm(44.0),
            width: ft_to_cm(32.0),
            thickness: ft_to_cm(50.0),
        }
    }

    fn assemble(&self) -> ScadObject {
        scad!(Union;{
            self.assemble_rows(),
            self.assemble_columns(),
        })
    }
}
