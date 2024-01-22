# Rust-eze spotlight
### Discovers a square area around the robot in an energy-efficient way, leaving the robot still.

#### *illuminate* discovers the tiles of a square area around the robot.
```rust
pub fn illuminate(
	robot: &mut impl Runnable,
	world: &mut World,
	distance: usize,
) -> Result<(), String>
```
#### Arguments
- robot: &mut impl Runnable
- world: &mut World
- distance: usize => distance from the robot to the edges of the square area.
#### Return
- Result<(), String> => Err(e) represents a possible error described by String e.

#### *calculate_illuminate_cost* calculates the energy required by *illuminate* to discover the square area with the given distance.
```rust
pub fn calculate_illuminate_cost(
	robot: &impl Runnable,
	world: &World,
	distance: usize,
) -> Result<usize, String>
```
#### Arguments
- robot: &impl Runnable
 - world: &World
- distance: usize => distance from the robot to the edges of the square area.
#### Return
- Result<usize, String> => Ok(cost) indicates the required energy, Err(e) represents a possible error described by String e.