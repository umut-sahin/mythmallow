#[doc(inline)]
pub use {
    super::plugin::MythmellowPlugin,
    crate::{
        camera::components::*,
        combat::components::*,
        configuration::resources::*,
        core::{
            resources::*,
            sets::*,
            states::*,
        },
        enemy::components::*,
        input::actions::*,
        map::components::*,
        movement::components::*,
        physics::{
            components::*,
            resources::*,
        },
        player::components::*,
        property::components::*,
        status_effect::components::*,
        ui::{
            game_over_menu::components::*,
            main_menu::components::*,
            pause_menu::components::*,
            widget::components::*,
        },
    },
};

pub(crate) use {
    bevy::{
        app::AppExit,
        ecs::schedule::ScheduleLabel,
        input::mouse::MouseMotion,
        prelude::*,
        sprite::MaterialMesh2dBundle,
        utils::HashSet,
        window::WindowFocused,
    },
    bevy_persistent::prelude::*,
    clap::Parser,
    leafwing_input_manager::{
        action_state::ActionData,
        buttonlike::ButtonState,
        prelude::*,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    smallvec::{
        smallvec,
        SmallVec,
    },
    std::{
        fmt::{
            self,
            Debug,
        },
        marker::PhantomData,
        path::PathBuf,
        time::Duration,
    },
    strum::IntoEnumIterator as _,
    strum_macros::EnumIter,
};

#[cfg(feature = "native")]
pub(crate) use {
    bevy::window::{
        PrimaryWindow,
        WindowMode,
    },
    bevy_persistent_windows::prelude::*,
    std::time::Instant,
};

#[cfg(feature = "wasm")]
pub(crate) use {
    instant::Instant,
    std::path::Path,
};
