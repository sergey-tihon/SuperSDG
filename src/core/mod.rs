use bevy::prelude::*;

pub mod menu;

/// Application states for the game
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Menu, // Start menu (initial state)
    InGame, // Playing the game
    Paused, // In-game pause menu
}

/// Maze complexity level affecting maze dimensions
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Complexity {
    Small, // 15x15
    #[default]
    Medium, // 30x30
    Large, // 60x60
}

impl Complexity {
    pub fn maze_size(self) -> (usize, usize) {
        match self {
            Complexity::Small => (15, 15),
            Complexity::Medium => (30, 30),
            Complexity::Large => (60, 60),
        }
    }

    pub fn next(self) -> Complexity {
        match self {
            Complexity::Small => Complexity::Medium,
            Complexity::Medium => Complexity::Large,
            Complexity::Large => Complexity::Small,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Complexity::Small => "15x15",
            Complexity::Medium => "30x30",
            Complexity::Large => "60x60",
        }
    }
}

/// Global game settings
#[derive(Resource, Default)]
pub struct GameSettings {
    pub complexity: Complexity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complexity_maze_sizes() {
        assert_eq!(Complexity::Small.maze_size(), (15, 15));
        assert_eq!(Complexity::Medium.maze_size(), (30, 30));
        assert_eq!(Complexity::Large.maze_size(), (60, 60));
    }

    #[test]
    fn complexity_cycles_through_all_levels() {
        let small = Complexity::Small;
        let medium = small.next();
        let large = medium.next();
        let back_to_small = large.next();

        assert_eq!(medium, Complexity::Medium);
        assert_eq!(large, Complexity::Large);
        assert_eq!(back_to_small, Complexity::Small);
    }

    #[test]
    fn complexity_labels() {
        assert_eq!(Complexity::Small.label(), "15x15");
        assert_eq!(Complexity::Medium.label(), "30x30");
        assert_eq!(Complexity::Large.label(), "60x60");
    }

    #[test]
    fn default_complexity_is_medium() {
        assert_eq!(Complexity::default(), Complexity::Medium);
    }

    #[test]
    fn default_game_settings_has_medium_complexity() {
        let settings = GameSettings::default();
        assert_eq!(settings.complexity, Complexity::Medium);
    }
}
