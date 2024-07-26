#![doc = include_str!("../README.md")]

/// Core of the game.
pub mod core {
    #[doc(inline)]
    pub use {
        mythmallow_core_dependencies as dependencies,
        mythmallow_core_plugins as plugins,
        mythmallow_core_resources as resources,
    };
}

/// Content of the game.
pub mod content {
    /// Enemies of the game.
    pub mod enemies {}
    /// Items of the game.
    pub mod items {}
    /// Modes of the game.
    pub mod modes {}
    /// Perks of the game.
    pub mod perks {}
    /// Players of the game.
    pub mod players {}
}
