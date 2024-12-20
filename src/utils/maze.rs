use crate::utils::c2::{C2Field, C2};

pub type Maze = (C2Field<bool>, C2, C2);

pub fn parse_maze(input: &str) -> Maze {
    let mut s = C2::new(-1, -1);
    let mut e = C2::new(-1, -1);

    let map = C2Field::from_string_indexed(input, &mut |c, ch| match ch {
        'S' => {
            s = c;
            true
        }
        'E' => {
            e = c;
            true
        }
        '.' => true,
        _ => false,
    });

    (map, s, e)
}
