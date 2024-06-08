use crate::{
    configuration::{
        commands::*,
        systems::*,
    },
    prelude::*,
};

/// Plugin for managing the configuration of the application.
pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<GeneralSettings>();
        app.register_type::<LocaleAssets>();
        app.register_type::<KeyBindings>();

        // Initialize configurations.
        GeneralSettings::initialize(app);
        KeyBindings::initialize(app);

        // Setup localization.
        {
            let general_setting = app.world.resource::<Persistent<GeneralSettings>>();

            let supported_locales = SupportedLocales::get();
            let locale_assets = LocaleAssets::default();
            let default_locale = DefaultLocale::get(&supported_locales);
            let current_locale = general_setting.locale().unwrap_or_else(|| default_locale.clone());
            let locale =
                Locale::new(current_locale.identifier()).with_default(default_locale.identifier());

            app.insert_resource(supported_locales);
            app.insert_resource(locale_assets);
            app.insert_resource(default_locale);
            app.insert_resource(locale);

            app.add_systems(OnEnter(LocalizationState::Loading), load_locale_assets);
            app.add_systems(
                Update,
                load_locales_folder.run_if(in_state(LocalizationState::Loading)).run_if(
                    |asset_server: Res<AssetServer>,
                     locale_assets: Option<Res<LocaleAssetHandles>>| {
                        if let Some(locale_assets) = locale_assets {
                            locale_assets.iter().all(|handle| {
                                matches!(
                                    asset_server.get_load_state(handle),
                                    None | Some(LoadState::Loaded | LoadState::Failed),
                                )
                            })
                        } else {
                            false
                        }
                    },
                ),
            );
            app.add_systems(
                Update,
                transition_to_ready
                    .run_if(in_state(LocalizationState::Loading))
                    .run_if(resource_exists::<LocalesFolder>),
            );
            app.add_systems(
                OnEnter(LocalizationState::Ready),
                transition_to_application.run_if(in_state(AppState::LoadingInitialLocalization)),
            );

            app.add_plugins(FluentPlugin);
        }

        // Add console commands.
        app.add_console_command::<LocaleCommand, _>(apply_locale_command);
    }
}
