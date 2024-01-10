#[doc(inline)]
pub use crate::{
    camera::components::*,
    combat::components::*,
    configuration::resources::*,
    core::{
        depths::*,
        resources::*,
        sets::*,
        states::*,
    },
    enemy::{
        components::*,
        interfaces::*,
        registry::*,
        resources::*,
        systems::follow_player,
    },
    input::actions::*,
    inventory::resources::*,
    items::{
        interfaces::*,
        registry::*,
    },
    map::{
        components::*,
        resources::*,
    },
    mode::{
        conditions::in_game_mode,
        interfaces::*,
        registry::*,
        resources::*,
    },
    movement::components::*,
    physics::layers::*,
    player::{
        components::*,
        interfaces::*,
        registry::*,
        resources::*,
    },
    plugin::MythmallowPlugin,
    property::components::*,
    status_effect::components::*,
    status_effect::systems::cooldown,
    ui::{
        diagnostics_overlay::components::*,
        enemy_selection_screen::components::*,
        game_mode_selection_screen::components::*,
        game_over_menu::components::*,
        main_menu::components::*,
        pause_menu::components::*,
        player_selection_screen::components::*,
        widget::components::*,
    },
};

pub mod utils {
    pub use crate::{
        combat::utils as combat,
        map::utils as map,
    };
}

#[doc(inline)]
pub use {
    bevy::{
        app::AppExit,
        diagnostic::{
            DiagnosticsStore,
            EntityCountDiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
        },
        ecs::{
            self as bevy_ecs,
            system::{
                EntityCommands,
                RunSystemOnce,
            },
        },
        input::mouse::MouseMotion,
        log::{
            self,
            LogPlugin,
        },
        prelude::*,
        reflect as bevy_reflect,
        sprite::MaterialMesh2dBundle,
        transform::TransformSystem,
        window::WindowFocused,
    },
    bevy_persistent::prelude::*,
    bevy_prng::ChaCha8Rng,
    bevy_rand::prelude::*,
    bevy_xpbd_2d::{
        math::*,
        plugins::PhysicsPlugins as XpbdPlugin,
        prelude::*,
    },
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
    smol_str::SmolStr,
    std::{
        any::{
            Any,
            TypeId,
        },
        cmp::Ordering,
        fmt::{
            self,
            Debug,
            Display,
        },
        marker::PhantomData,
        num::NonZeroU8,
        ops::{
            Deref,
            DerefMut,
            Index,
        },
        path::PathBuf,
        sync::{
            Arc,
            Mutex,
        },
        time::Duration,
    },
    strum::IntoEnumIterator,
    strum_macros::EnumIter,
    typed_builder::TypedBuilder,
};

#[cfg(feature = "native")]
#[doc(inline)]
pub use {
    bevy::window::{
        ExitCondition,
        PrimaryWindow,
        WindowMode,
    },
    bevy_persistent_windows::prelude::*,
    std::time::Instant,
};

#[cfg(feature = "wasm")]
#[doc(inline)]
pub use {
    instant::Instant,
    std::path::Path,
};

#[cfg(feature = "bevy_editor_pls")]
#[doc(inline)]
pub use bevy_editor_pls::prelude::*;
