use robotics_lib::interface::robot_map;
use robotics_lib::runner::Runnable;
use robotics_lib::world::World;

fn calculate_view_cost(distance: usize) -> usize {
    match distance {
        0..=1 => 0,
        _ => distance * 3,
    }
}

fn calculate_discover_tiles_cost(world: &World, tiles: Vec<(usize, usize)>) -> usize {
    let mut ret = 0;

    if let Some(map) = robot_map(world) {
        let size = map.len();

        for (row, col) in tiles {
            if row < size && col < size && map[row][col].is_none() {
                ret += 1;
            }
        }
    }

    ret
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

pub(crate) fn calculate_illuminate_cost(
    robot: &impl Runnable,
    world: &World,
    distance: usize,
) -> Result<usize, String> {
    match robot_map(world) {
        None => Err(String::from("Map not visible!")),
        Some(map) => {
            let size = map.len();

            let (robot_row, robot_col) = (
                robot.get_coordinate().get_row(),
                robot.get_coordinate().get_col(),
            );

            let mut ret = 0;

            let up_distance = std::cmp::min(distance, robot_row);
            let down_distance = std::cmp::min(distance, size - 1 - robot_row);
            let right_distance = std::cmp::min(distance, size - 1 - robot_col);
            let left_distance = std::cmp::min(distance, robot_col);

            ret += calculate_view_cost(up_distance);
            ret += calculate_view_cost(down_distance);
            ret += calculate_view_cost(right_distance);
            ret += calculate_view_cost(left_distance);

            let up_left_corner = get_up_left_corner(robot_row, robot_col, distance);
            let bottom_right_corner = get_bottom_right_corner(robot_row, robot_col, distance, size);

            let mut tiles: Vec<_> = Vec::new();

            for row in (up_left_corner.0)..=(bottom_right_corner.0) {
                for col in (up_left_corner.1)..=(bottom_right_corner.1) {
                    if row + 1 < robot_row && row  > robot_row + 1 {
                        if col + 1 < robot_col && col > robot_col + 1 {
                            tiles.push((row, col));
                        }
                    }
                }
            }

            ret += calculate_discover_tiles_cost(world, tiles);

            Ok(ret)
        }
    }
}
