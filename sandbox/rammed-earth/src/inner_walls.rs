use dimdraw::{ObjectAssembler, ObjectDescriptor};
use parts::common_functions::*;
use parts::Wall;
use scad::*;

use config::*;

qstruct!(InnerWalls(color: Option<&'static str>) {
    l2: Wall = Wall::new(
        INNER_WALL_L2_LENGTH,
        INNER_WALL_WIDTH,
        INNER_WALL_THICKNESS,
        color,
        vec!()),
    l4: Wall = Wall::new(
        INNER_WALL_L4_LENGTH,
        INNER_WALL_WIDTH,
        INNER_WALL_THICKNESS,
        color,
        vec!()),
    l5: Wall = Wall::new(
        INNER_WALL_L5_LENGTH,
        INNER_WALL_WIDTH,
        INNER_WALL_THICKNESS,
        color,
        vec!()),
    l6: Wall = Wall::new(
        INNER_WALL_L6_LENGTH,
        INNER_WALL_WIDTH,
        INNER_WALL_THICKNESS,
        color,
        vec!()),
    l8: Wall = Wall::new(
        INNER_WALL_L8_LENGTH,
        INNER_WALL_WIDTH,
        INNER_WALL_THICKNESS,
        color,
        vec!()),
    l10: Wall = Wall::new(
        INNER_WALL_L10_LENGTH,
        INNER_WALL_WIDTH,
        INNER_WALL_THICKNESS,
        color,
        vec!()),
});

impl InnerWalls {
    fn assemble_music_room(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // rows
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(20.0);
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(36.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l4.assemble_xaligned()
        }));
        let x_offset = ft_to_cm(42.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8.assemble_xaligned()
        }));

        // columns
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(36.0) - self.l6.thickness();
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(20.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l6.assemble_yaligned()
        }));
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(26.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l6.assemble_yaligned()
        }));

        parent
    }

    fn assemble_bed_room(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // rows
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(8.0) - self.l8.thickness();
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(36.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l4.assemble_xaligned()
        }));

        // columns
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(32.0) - self.l10.thickness();
        let y_offset = OUTER_WALL_THICKNESS;
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10.assemble_yaligned()
        }));
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(10.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10.assemble_yaligned()
        }));

        parent
    }

    fn assemble_guest_bath_room(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // rows
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(6.0);
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(26.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l5.assemble_xaligned()
        }));

        // columns
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(26.0) - self.l4.thickness();
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(2.0) + self.l4.thickness();
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l4.assemble_yaligned()
        }));

        parent
    }

    fn assemble_office_room(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // rows
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(21.0);
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(20.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8.assemble_xaligned()
        }));

        // columns
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(20.0);
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(21.0) + self.l4.thickness();
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l10.assemble_yaligned()
        }));
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(28.0) - self.l4.thickness();
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(21.0) + self.l4.thickness();
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l2.assemble_yaligned()
        }));
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(32.0) - self.l4.thickness();
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(24.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8.assemble_yaligned()
        }));

        parent
    }

    fn assemble_living_room(&self) -> ScadObject {
        let mut parent = scad!(Union);

        // columns
        let x_offset = OUTER_WALL_THICKNESS + ft_to_cm(16.0) - self.l8.thickness();
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(16.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8.assemble_yaligned()
        }));
        let y_offset = OUTER_WALL_THICKNESS + ft_to_cm(24.0);
        parent.add_child(scad!(Translate(vec3(x_offset, y_offset, 0.0));{
            self.l8.assemble_yaligned()
        }));

        parent
    }
}

impl ObjectAssembler for InnerWalls {
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
            self.assemble_music_room(),
            self.assemble_bed_room(),
            self.assemble_guest_bath_room(),
            self.assemble_office_room(),
            self.assemble_living_room(),
        })
    }
}