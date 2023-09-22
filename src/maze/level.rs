use bevy::prelude::*;
use rand::seq::SliceRandom;
use std::fmt;

pub struct MazeLevelPlugin;

impl Plugin for MazeLevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MazeLevel::new(20, 20));
    }
}

#[derive(Resource)]
pub struct MazeLevel {
    pub width: usize,
    pub height: usize,
    pub map: Vec<Vec<char>>,
}

impl MazeLevel {
    fn new(x: usize, y: usize) -> MazeLevel {
        let width = (2 * x) + 1;
        let height = (2 * y) + 1;

        let mut maze = MazeLevel {
            width,
            height,
            map: vec![vec!['#'; height]; width],
        };

        maze.generate_maze(1, 1);
        maze
    }

    fn generate_maze(&mut self, x: usize, y: usize) {
        let mut rng = rand::thread_rng();
        let directions: [(&i32, &i32); 4] = [(&0, &1), (&1, &0), (&0, &-1), (&-1, &0)];
        let dir_choices: Vec<_> = directions
            .choose_multiple(&mut rng, directions.len())
            .cloned()
            .collect();

        for (&dx, &dy) in dir_choices.iter() {
            let nx = x as i32 + 2 * dx;
            let ny = y as i32 + 2 * dy;
            let nx = nx as usize;
            let ny = ny as usize;

            if nx < self.width - 1 && ny < self.height - 1 && self.map[nx][ny] == '#' {
                self.map[(x as i32 + dx) as usize][(y as i32 + dy) as usize] = ' ';
                self.map[nx][ny] = ' ';
                self.generate_maze(nx, ny);
            }
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
    use super::MazeLevel;

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
}
