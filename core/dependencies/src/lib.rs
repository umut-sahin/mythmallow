//! Dependencies of the game.

#[doc(inline)]
pub use {
    bevy::{
        self,
        diagnostic::{
            EntityCountDiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
        },
        ecs as bevy_ecs,
        log::{
            self,
            LogPlugin,
        },
        prelude::*,
        reflect as bevy_reflect,
    },
    bevy_persistent::{
        self,
        prelude::*,
    },
    clap::{
        self,
        CommandFactory,
        Parser,
    },
    std::{
        fmt::{
            self,
            Display,
        },
        path::{
            Path,
            PathBuf,
        },
    },
};

#[cfg(not(target_family = "wasm"))]
#[doc(inline)]
pub use {
    bevy::window::PrimaryWindow,
    bevy_persistent_windows::prelude::*,
    dirs::{
        self,
    },
};

#[cfg(target_family = "wasm")]
#[doc(inline)]
pub use {
    console_error_panic_hook::{
        self,
    },
    web_sys::{
        self,
    },
};
