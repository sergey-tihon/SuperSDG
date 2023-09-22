use bevy::prelude::*;

pub struct MazeLevelPlugin;

impl Plugin for MazeLevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MazeLevel::new(20, 20));
    }
}

#[derive(Resource)]
pub struct MazeLevel {
    pub map: Vec<&'static str>,
}

impl MazeLevel {
    fn new(_x: u8, _y: u8) -> MazeLevel {
        let map = vec![
            "####################",
            "#                  #",
            "#  ##### ######### #",
            "#      #      #    #",
            "#  ########## ###  #",
            "#        ####      #",
            "#  ##### ######### #",
            "#      #      #    #",
            "#  ########## ###  #",
            "#    ###           #",
            "#  ##### ######### #",
            "#      #      #  # #",
            "#  ########## #### #",
            "#     #####        #",
            "#                # #",
            "#  ##### ######### #",
            "#    ###      #    #",
            "#  ########## ###  #",
            "#                  #",
            "####################",
        ];

        MazeLevel { map }
    }
}

#[cfg(test)]
mod test {
    use super::MazeLevel;

    #[test]
    fn border_exist() {
        let level = MazeLevel::new(20, 20);

        for (z, &s) in level.map.iter().enumerate() {
            for (x, c) in s.chars().enumerate() {
                if z == 0 || x == 0 || z == level.map.len() - 1 || x == s.len() - 1 {
                    assert_eq!(c, '#');
                }
            }
        }
    }
}
