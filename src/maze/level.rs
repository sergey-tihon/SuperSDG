use bevy::prelude::*;

pub struct MazeLevelPlugin;

impl Plugin for MazeLevelPlugin {
    fn build(&self, app: &mut App) {
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

        app.insert_resource(MazeLevel { map });
    }
}

#[derive(Resource)]
pub struct MazeLevel {
    pub map: Vec<&'static str>,
}
