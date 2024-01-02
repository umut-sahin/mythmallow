#![doc = include_str!("../README.md")]
pub use mythmallow_game::*;
pub mod content {
    pub mod items {
        pub mod greek {
            pub use mythmallow_items_greek::*;
        }
    }
    pub mod modes {
        pub mod survival {
            pub use mythmallow_mode_survival::*;
        }
    }
    pub mod players {
        pub mod greek {
            pub use mythmallow_players_greek::*;
        }
    }
}
