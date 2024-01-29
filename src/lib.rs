use robotics_lib::interface::{
    discover_tiles, one_direction_view, robot_map, robot_view, Direction, Tools,
};
use robotics_lib::runner::Runnable;
use robotics_lib::world::World;
use utils::{
    calculate_discover_tiles_cost, calculate_distance, calculate_view_cost,
    get_bottom_right_corner, get_up_left_corner,
};

mod utils;

#[derive(Default)]
pub struct Spotlight {}

impl Tools for Spotlight {}

impl Spotlight {
    /// illuminate discovers the tiles of a square area around the robot.
    ///
    /// # Arguments
    /// - robot: &impl Runnable
    /// - world: &World
    /// - distance: usize => distance from the robot to the edges of the square area.
    ///
    /// # Return
    /// - Result<(), String> => Err(e) represents a possible error described by String e.
    pub fn illuminate(
        robot: &mut impl Runnable,
        world: &mut World,
        distance: usize,
    ) -> Result<(), String> {
        match robot_map(world) {
            None => Err(String::from("Map not visible!")),
            Some(mut map) => {
                let size = map.len();
                let distance = std::cmp::min(distance, size);

                let (robot_row, robot_col) = (
                    robot.get_coordinate().get_row(),
                    robot.get_coordinate().get_col(),
                );

                if distance <= 1 {
                    if distance == 1 {
                        robot_view(robot, world);
                    }

                    return Ok(());
                }

                match Spotlight::calculate_illuminate_cost(robot, world, distance) {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_) => {
                        robot_view(robot, world);

                        if one_direction_view(robot, world, Direction::Right, distance).is_err() {
                            return Err(String::from(
                                "Error while calling one_direction_view interface!",
                            ));
                        }

                        if one_direction_view(robot, world, Direction::Down, distance).is_err() {
                            return Err(String::from(
                                "Error while calling one_direction_view interface!",
                            ));
                        }

                        if one_direction_view(robot, world, Direction::Left, distance).is_err() {
                            return Err(String::from(
                                "Error while calling one_direction_view interface!",
                            ));
                        }

                        if one_direction_view(robot, world, Direction::Up, distance).is_err() {
                            return Err(String::from(
                                "Error while calling one_direction_view interface!",
                            ));
                        }

                        map = robot_map(world).unwrap();

                        let up_left_corner = get_up_left_corner(robot_row, robot_col, distance);
                        let bottom_right_corner =
                            get_bottom_right_corner(robot_row, robot_col, distance, size);

                        let mut tiles = Vec::new();

                        for row in (up_left_corner.0)..=(bottom_right_corner.0) {
                            for col in (up_left_corner.1)..=(bottom_right_corner.1) {
                                if map[row][col].is_none() {
                                    tiles.push((row, col));
                                }
                            }
                        }

                        tiles.sort_by(|a, b| {
                            let a_distance = calculate_distance(*a, (robot_row, robot_col));
                            let b_distance = calculate_distance(*b, (robot_row, robot_col));

                            if a_distance != b_distance {
                                a_distance.cmp(&b_distance)
                            } else if a.0 != b.0 {
                                a.0.cmp(&b.0)
                            } else {
                                a.1.cmp(&b.1)
                            }
                        });

                        for (row, col) in tiles.iter() {
                            if let Err(_) = discover_tiles(robot, world, &vec![(*row, *col)]) {
                                return Err(String::from(
                                    "Error while calling discover_tiles interface!",
                                ));
                            }
                        }
                    }
                }

                Ok(())
            }
        }
    }

    /// calculate_illuminate_cost calculates the energy required by illuminate to discover the square area with the given distance.
    ///
    /// # Arguments
    /// - robot: &impl Runnable
    /// - world: &World
    /// - distance: usize => distance from the robot to the edges of the square area.
    ///
    /// # Return
    /// - Result<usize, String> => Ok(cost) indicates the required energy, Err(e) represents a possible error described by String e.
    pub fn calculate_illuminate_cost(
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
                let left_distance = std::cmp::min(distance, robot_col);
                let down_distance = std::cmp::min(distance, size - 1 - robot_row);
                let right_distance = std::cmp::min(distance, size - 1 - robot_col);

                ret += calculate_view_cost(up_distance);
                ret += calculate_view_cost(left_distance);
                ret += calculate_view_cost(down_distance);
                ret += calculate_view_cost(right_distance);

                let up_left_corner = get_up_left_corner(robot_row, robot_col, distance);
                let bottom_right_corner =
                    get_bottom_right_corner(robot_row, robot_col, distance, size);

                let mut tiles = Vec::new();

                for row in (up_left_corner.0)..=(bottom_right_corner.0) {
                    for col in (up_left_corner.1)..=(bottom_right_corner.1) {
                        if row > robot_row + 1 || (robot_row > 1 && row < robot_row - 1) {
                            if col > robot_col + 1 || (robot_col > 1 && col < robot_col - 1) {
                                tiles.push((row, col));
                            }
                        }
                    }
                }

                ret += calculate_discover_tiles_cost(&map, tiles);

                Ok(ret)
            }
        }
    }
}
