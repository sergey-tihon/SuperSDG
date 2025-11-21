use bevy::prelude::*;
use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt;
use std::ops::Add;

/// Coordinate convention: (x, y) where x is the column (horizontal), y is the row (vertical).
/// All coordinates are 0-based, with (0,0) at the top-left corner of the maze.
/// Vec2i stores coordinates as (i32, i32) to allow for negative values during calculations,
/// but all indexing into the map must be checked for non-negativity and bounds.
pub struct MazeLevelPlugin;

impl Plugin for MazeLevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MazeLevel::new(20, 20));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2i(pub i32, pub i32);

impl std::ops::Add<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn add(self, rhs: Vec2i) -> Self::Output {
        Vec2i(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Vec2i {
    pub fn new(position: (usize, usize)) -> Self {
        Vec2i(position.0 as i32, position.1 as i32)
    }

    pub fn get_next(&self, direction_index: usize) -> Vec2i {
        self.add(Directions::get_2d(direction_index))
    }

    /// Returns Some((x, y)) as usize if both coordinates are non-negative, otherwise None.
    pub fn to_usize(self) -> Option<(usize, usize)> {
        if self.0 >= 0 && self.1 >= 0 {
            Some((self.0 as usize, self.1 as usize))
        } else {
            None
        }
    }
}

impl From<Vec2i> for Vec3 {
    fn from(val: Vec2i) -> Self {
        Vec3::new(val.0 as f32 + 0.5, 0.5, val.1 as f32 + 0.5)
    }
}

pub struct Directions;

impl Directions {
    pub const DIRECTIONS: [Vec2i; 4] = [Vec2i(0, -1), Vec2i(1, 0), Vec2i(0, 1), Vec2i(-1, 0)];

    pub fn get_2d(direction_index: usize) -> Vec2i {
        Self::DIRECTIONS[direction_index]
    }

    pub fn get_3d(direction_index: usize) -> Vec3 {
        let dir = Self::get_2d(direction_index);
        Vec3::new(dir.0 as f32, 0.0, dir.1 as f32)
    }

    pub fn get_closest(camera_forward: Vec3) -> usize {
        let mut base_index = 0;
        let mut base_cosine = f32::MIN;
        for index in 0..Directions::DIRECTIONS.len() {
            let cosine = Directions::get_3d(index).dot(camera_forward);
            if cosine > base_cosine {
                base_cosine = cosine;
                base_index = index;
            }
        }
        base_index
    }
}

#[derive(Resource)]
pub struct MazeLevel {
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<char>>,
    pub player_position: Vec2i,
    pub exit_position: Vec2i,
}

impl MazeLevel {
    pub fn new(x: usize, y: usize) -> MazeLevel {
        let width = (2 * x) + 1;
        let height = (2 * y) + 1;

        let mut maze = MazeLevel {
            width,
            height,
            map: vec![vec!['#'; width]; height],
            player_position: Vec2i(0, 0),
            exit_position: Vec2i(0, 0),
        };

        maze.generate_maze(1, 1);

        let (start, exit) = maze.random_player_and_exit_positions();
        maze.player_position = Vec2i::new(start);
        maze.exit_position = Vec2i::new(exit);

        maze
    }

    fn generate_maze(&mut self, x: usize, y: usize) {
        let mut rng = rand::rng();
        let directions: [(&i32, &i32); 4] = [(&0, &1), (&1, &0), (&0, &-1), (&-1, &0)];
        let dir_choices: Vec<_> = directions
            .choose_multiple(&mut rng, directions.len())
            .cloned()
            .collect();

        for &(&dx, &dy) in dir_choices.iter() {
            let nx = x as i32 + 2 * dx;
            let ny = y as i32 + 2 * dy;
            // Only proceed if nx, ny are non-negative and within bounds
            if nx > 0 && ny > 0 {
                let nx_usize = nx as usize;
                let ny_usize = ny as usize;
                if nx_usize < self.width - 1
                    && ny_usize < self.height - 1
                    && self.map[ny_usize][nx_usize] == '#'
                {
                    let wall_x = (x as i32 + dx) as usize;
                    let wall_y = (y as i32 + dy) as usize;
                    self.map[wall_y][wall_x] = ' ';
                    self.map[ny_usize][nx_usize] = ' ';
                    self.generate_maze(nx_usize, ny_usize);
                }
            }
        }
    }

    /// Returns random start and exit positions using BFS to find maximum distance.
    /// Uses coordinate convention: (x, y) where x=column, y=row.
    /// Map access is map[y][x] and visited array is accessed as visited[y][x].
    fn random_player_and_exit_positions(&self) -> ((usize, usize), (usize, usize)) {
        fn bfs(maze: &MazeLevel, start_x: usize, start_y: usize) -> ((usize, usize), usize) {
            let mut visited = vec![vec![false; maze.width]; maze.height];
            let mut queue = VecDeque::new();
            visited[start_y][start_x] = true;
            queue.push_back((start_x, start_y, 0));

            let mut max_distance = 0;
            let mut farthest_cell = (start_x, start_y);

            while let Some((x, y, dist)) = queue.pop_front() {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && ny >= 0 {
                        let nx_usize = nx as usize;
                        let ny_usize = ny as usize;
                        if nx_usize < maze.width
                            && ny_usize < maze.height
                            && maze.map[ny_usize][nx_usize] == ' '
                            && !visited[ny_usize][nx_usize]
                        {
                            visited[ny_usize][nx_usize] = true;
                            queue.push_back((nx_usize, ny_usize, dist + 1));
                            if dist + 1 > max_distance {
                                max_distance = dist + 1;
                                farthest_cell = (nx_usize, ny_usize);
                            }
                        }
                    }
                }
            }

            (farthest_cell, max_distance)
        }

        let mut rng = rand::rng();
        let empty_cells: Vec<(usize, usize)> = (1..self.width)
            .flat_map(|x| (1..self.height).map(move |y| (x, y)))
            .filter(|&(x, y)| self.map[y][x] == ' ')
            .collect();
        let &(start_x, start_y) = empty_cells.choose(&mut rng).unwrap();
        let (exit, _) = bfs(self, start_x, start_y);

        ((start_x, start_y), exit)
    }

    /// Safely checks if a cell is empty, returning false if out of bounds or negative.
    pub fn is_cell_empty(&self, position: Vec2i) -> bool {
        if let Some((x, y)) = position.to_usize() {
            x < self.width && y < self.height && self.map[y][x] != '#'
        } else {
            false
        }
    }
}

impl fmt::Display for MazeLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.map {
            for ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{MazeLevel, Vec2i};

    #[test]
    fn border_exist() {
        let level = MazeLevel::new(20, 20);

        for (z, s) in level.map.iter().enumerate() {
            for (x, &c) in s.iter().enumerate() {
                if z == 0 || x == 0 || z == level.map.len() - 1 || x == s.len() - 1 {
                    assert_eq!(c, '#');
                }
            }
        }
    }

    #[test]
    fn maze_generation() {
        let level = MazeLevel::new(20, 20);
        let (start, exit) = level.random_player_and_exit_positions();

        assert_eq!(level.map[start.1][start.0], ' ');
        assert_eq!(level.map[exit.1][exit.0], ' ');
    }

    #[test]
    fn is_cell_empty_correctly_identifies_empty_cells() {
        let level = MazeLevel::new(20, 20);

        // Check all cells in the maze
        for x in 0..level.width {
            for y in 0..level.height {
                let position = Vec2i(x as i32, y as i32);
                let is_empty = level.is_cell_empty(position);
                let cell_value = level.map[y][x];

                if cell_value == ' ' {
                    assert!(is_empty, "Cell at ({}, {}) should be empty", x, y);
                } else {
                    assert!(!is_empty, "Cell at ({}, {}) should not be empty", x, y);
                }
            }
        }
    }

    #[test]
    fn player_placement_is_always_in_empty_cell() {
        let level = MazeLevel::new(20, 20);
        let player_pos = level.player_position;

        // Convert to usize for map access
        let x = player_pos.0 as usize;
        let y = player_pos.1 as usize;

        // Check that the player is placed in an empty cell
        assert_eq!(
            level.map[y][x], ' ',
            "Player should be placed in an empty cell"
        );

        // Check that is_cell_empty returns true for the player position
        assert!(
            level.is_cell_empty(player_pos),
            "is_cell_empty should return true for player position"
        );
    }

    #[test]
    fn movement_in_all_directions() {
        let level = MazeLevel::new(20, 20);
        let player_pos = level.player_position;

        // Test movement in all four directions
        for direction in 0..4 {
            let next_pos = player_pos.get_next(direction);

            // Use safe indexing for test
            if let Some((x, y)) = next_pos.to_usize() {
                if x < level.width && y < level.height && level.map[y][x] == ' ' {
                    assert!(
                        level.is_cell_empty(next_pos),
                        "Movement to ({}, {}) should be allowed",
                        next_pos.0,
                        next_pos.1
                    );
                } else {
                    assert!(
                        !level.is_cell_empty(next_pos),
                        "Movement to ({}, {}) should not be allowed",
                        next_pos.0,
                        next_pos.1
                    );
                }
            } else {
                // Out of bounds or negative, must not be empty
                assert!(
                    !level.is_cell_empty(next_pos),
                    "Movement to ({}, {}) should not be allowed (out of bounds)",
                    next_pos.0,
                    next_pos.1
                );
            }
        }
    }
}
