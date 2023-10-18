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
        mode::{
            interfaces::*,
            registry::*,
            resources::*,
        },
        movement::components::*,
        physics::{
            components::*,
            resources::*,
        },
        player::components::*,
        property::components::*,
        status_effect::components::*,
        ui::{
            game_mode_selection_screen::components::*,
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
        log,
        prelude::*,
        sprite::MaterialMesh2dBundle,
        utils::HashSet,
        window::WindowFocused,
    },
    bevy_persistent::prelude::*,
    bevy_prng::ChaCha8Rng,
    bevy_rand::prelude::*,
    clap::Parser,
    leafwing_input_manager::{
        action_state::ActionData,
        buttonlike::ButtonState,
        prelude::*,
    },
    rand::prelude::*,
    serde::{
        Deserialize,
        Serialize,
    },
    smallvec::{
        smallvec,
        SmallVec,
    },
    std::{
        any::TypeId,
        fmt::{
            self,
            Debug,
        },
        marker::PhantomData,
        ops::{
            Deref,
            DerefMut,
        },
        path::PathBuf,
        sync::Mutex,
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
