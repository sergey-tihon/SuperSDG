use bevy::prelude::*;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
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
    pub start: (i32, i32),
    pub exit: (i32, i32),
}

impl MazeLevel {
    fn new(x: usize, y: usize) -> MazeLevel {
        let width = (2 * x) + 1;
        let height = (2 * y) + 1;

        let mut maze = MazeLevel {
            width,
            height,
            map: vec![vec!['#'; height]; width],
            start: (0, 0),
            exit: (0, 0),
        };

        maze.generate_maze(1, 1);

        let (start, exit) = maze.random_player_and_exit_positions();
        maze.start = (start.0 as i32, start.1 as i32);
        maze.exit = (exit.0 as i32, exit.1 as i32);

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

    fn random_player_and_exit_positions(&self) -> ((usize, usize), (usize, usize)) {
        fn bfs(maze: &MazeLevel, start_x: usize, start_y: usize) -> ((usize, usize), usize) {
            let mut visited = vec![vec![false; maze.height]; maze.width];
            let mut queue = VecDeque::new();
            visited[start_x][start_y] = true;
            queue.push_back((start_x, start_y, 0));

            let mut max_distance = 0;
            let mut farthest_cell = (start_x, start_y);

            while let Some((x, y, dist)) = queue.pop_front() {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                    let nx = (x as i32 + dx) as usize;
                    let ny = (y as i32 + dy) as usize;
                    if nx < maze.width
                        && ny < maze.height
                        && maze.map[nx][ny] == ' '
                        && !visited[nx][ny]
                    {
                        visited[nx][ny] = true;
                        queue.push_back((nx, ny, dist + 1));
                        if dist > max_distance {
                            max_distance = dist + 1;
                            farthest_cell = (nx, ny);
                        }
                    }
                }
            }

            (farthest_cell, max_distance)
        }

        let mut rng = rand::thread_rng();
        let empty_cells: Vec<(usize, usize)> = (1..self.width)
            .flat_map(|x| (1..self.height).map(move |y| (x, y)))
            .filter(|&(x, y)| self.map[x][y] == ' ')
            .collect();
        let &(start_x, start_y) = empty_cells.choose(&mut rng).unwrap();
        let (exit, _) = bfs(self, start_x, start_y);

        ((start_x, start_y), exit)
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

    #[test]
    fn maze_generation() {
        let level = MazeLevel::new(20, 20);
        let (start, exit) = level.random_player_and_exit_positions();

        assert_eq!(level.map[start.0][start.1], ' ');
        assert_eq!(level.map[exit.0][exit.1], ' ');
    }
}
