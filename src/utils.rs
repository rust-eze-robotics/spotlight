use robotics_lib::world::tile::Tile;

pub(crate) fn calculate_view_cost(distance: usize) -> usize {
    match distance {
        0..=1 => 0,
        _ => distance * 3,
    }
}

pub(crate) fn calculate_discover_tiles_cost(
    map: &Vec<Vec<Option<Tile>>>,
    tiles: Vec<(usize, usize)>,
) -> usize {
    let mut ret = 0;

    let size = map.len();

    for (row, col) in tiles {
        if row < size && col < size && map[row][col].is_none() {
            ret += 1;
        }
    }

    ret * 3
}

pub(crate) fn get_up_left_corner(
    robot_row: usize,
    robot_col: usize,
    distance: usize,
) -> (usize, usize) {
    let mut up_left_corner = (0, 0);

    if robot_row >= distance {
        up_left_corner.0 = robot_row - distance;
    }

    if robot_col >= distance {
        up_left_corner.1 = robot_col - distance;
    }

    up_left_corner
}

pub(crate) fn get_bottom_right_corner(
    robot_row: usize,
    robot_col: usize,
    distance: usize,
    world_size: usize,
) -> (usize, usize) {
    let mut bottom_right_corner = (robot_row + distance, robot_col + distance);

    bottom_right_corner.0 = std::cmp::min(bottom_right_corner.0, world_size - 1);
    bottom_right_corner.1 = std::cmp::min(bottom_right_corner.1, world_size - 1);

    bottom_right_corner
}
