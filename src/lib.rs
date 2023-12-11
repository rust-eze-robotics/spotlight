use robotics_lib::interface::{discover_tiles, one_direction_view, robot_map, Direction, Tools};
use robotics_lib::runner::Runnable;
use robotics_lib::world::World;
use utils::{calculate_illuminate_cost, get_bottom_right_corner, get_up_left_corner};

mod utils;

#[derive(Default)]
pub struct Spotlight {}

impl Tools for Spotlight {}

impl Spotlight {
    pub fn illuminate(
        &self,
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

                match calculate_illuminate_cost(robot, world, distance) {
                    Err(e) => {
                        return Err(e);
                    }
                    Ok(_) => {
                        if let Err(_) = one_direction_view(robot, world, Direction::Right, distance)
                        {
                            return Err(String::from(
                                "Error while calling one_direction_view interface!",
                            ));
                        }

                        if let Err(_) = one_direction_view(robot, world, Direction::Down, distance)
                        {
                            return Err(String::from(
                                "Error while calling one_direction_view interface!",
                            ));
                        }

                        if let Err(_) = one_direction_view(robot, world, Direction::Left, distance)
                        {
                            return Err(String::from(
                                "Error while calling one_direction_view interface!",
                            ));
                        }

                        if let Err(_) = one_direction_view(robot, world, Direction::Up, distance) {
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
}
