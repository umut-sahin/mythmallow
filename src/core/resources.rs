use crate::prelude::*;

/// Result of the run.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect, Resource)]
pub enum GameResult {
    Won,
    Lost,
}
