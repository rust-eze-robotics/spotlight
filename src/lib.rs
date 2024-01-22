use robotics_lib::interface::{discover_tiles, one_direction_view, robot_map, Direction, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::world::World;
use utils::{
    calculate_discover_tiles_cost, calculate_view_cost, get_bottom_right_corner, get_up_left_corner,
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
            Some(map) => {
                let size = map.len();
                let distance = std::cmp::min(distance, size);

                let (robot_row, robot_col) = (
                    robot.get_coordinate().get_row(),
                    robot.get_coordinate().get_col(),
                );

                if distance <= 1 {
                    return Ok(());
                }

                match Spotlight::calculate_illuminate_cost(robot, world, distance) {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_) => {
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

                        let up_left_corner = get_up_left_corner(robot_row, robot_col, distance);
                        let bottom_right_corner =
                            get_bottom_right_corner(robot_row, robot_col, distance, size);

                        for row in (up_left_corner.0)..=(bottom_right_corner.0) {
                            for col in (up_left_corner.1)..=(bottom_right_corner.1) {
                                if map[row][col].is_none() {
                                    if let Err(_) = discover_tiles(robot, world, &vec![(row, col)])
                                    {
                                        return Err(String::from(
                                            "Error while calling discover_tiles interface!",
                                        ));
                                    }
                                }
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

                let mut tiles: Vec<_> = Vec::new();

                for row in (up_left_corner.0)..=(bottom_right_corner.0) {
                    for col in (up_left_corner.1)..=(bottom_right_corner.1) {
                        tiles.push((row, col));
                    }
                }

                ret += calculate_discover_tiles_cost(world, tiles);

                Ok(ret)
            }
        }
    }
}
