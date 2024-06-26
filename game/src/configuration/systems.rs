use crate::{
    configuration::commands::*,
    prelude::*,
};


/// Applies the locale console commands.
pub fn apply_locale_command(
    mut commands: Commands,
    registered_systems: Res<RegisteredSystems>,
    supported_locales: Res<SupportedLocales>,
    locale: ResMut<Locale>,
    mut command: ConsoleCommand<LocaleCommand>,
) {
    if let Some(Ok(LocaleCommand { subcommand })) = command.take() {
        match subcommand {
            LocaleCommands::List => {
                for (i, locale) in supported_locales.iter().enumerate() {
                    reply!(command, "{}) {}", i + 1, locale.to_string());
                }
            },
            LocaleCommands::Show => {
                reply!(command, "{}", locale.requested);
            },
            LocaleCommands::Set { locale: requested } => {
                match requested.parse::<LanguageIdentifier>() {
                    Ok(new_locale) if supported_locales.contains(&new_locale) => {
                        commands.run_system_with_input(
                            registered_systems.configuration.set_locale,
                            new_locale,
                        );
                        reply!(command, "Set.");
                    },
                    _ => reply!(command, "Requested locale isn't available."),
                }
            },
        }

        reply!(command, "");
    }
}


/// Loads locale assets.
pub fn load_locale_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    locale_assets: Res<LocaleAssets>,
    locale: Res<Locale>,
) {
    let mut locale_resource_handles = Vec::with_capacity(locale_assets.len());

    for asset in locale_assets.iter() {
        let path = format!("locales/{}/{}", locale.requested, asset);
        log::info!("loading asset {}", path);
        locale_resource_handles.push(asset_server.load::<ResourceAsset>(path));
    }

    commands.insert_resource(LocaleResourceHandles(locale_resource_handles));
}

/// Loads locales folder.
pub fn load_locales_folder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut bundle_assets: ResMut<Assets<BundleAsset>>,
    resource_assets: ResMut<Assets<ResourceAsset>>,
    mut loaded_folders: ResMut<Assets<LoadedFolder>>,
    locale: Res<Locale>,
    mut locale_resource_handles: ResMut<LocaleResourceHandles>,
) {
    let mut fluent_bundle = FluentBundle::new_concurrent(vec![locale.requested.clone()]);
    fluent_bundle.set_use_isolating(false);

    let mut handles = Vec::with_capacity(1 + locale_resource_handles.len());
    for resource in std::mem::take(locale_resource_handles.deref_mut()).0 {
        if asset_server.get_load_state(&resource) == Some(LoadState::Loaded) {
            handles.push(resource.clone().untyped());

            if let Some(resource_asset) = resource_assets.get(&resource) {
                if let Err(errors) = fluent_bundle.add_resource(resource_asset.0.clone()) {
                    for error in errors {
                        if let Some(path) = asset_server.get_path(&resource) {
                            log::warn!("in {}: {}", path, error);
                        } else {
                            log::warn!("in locales/{}/???: {}", locale.requested, error);
                        }
                    }
                }
            }
        }
    }

    let bundle_asset = BundleAsset(Arc::new(fluent_bundle));
    handles.push(bundle_assets.add(bundle_asset).untyped());

    commands.remove_resource::<LocaleResourceHandles>();
    commands.insert_resource(LocalesFolder(loaded_folders.add(LoadedFolder { handles })));
}

/// Transitions to the ready when locales folder is loaded.
pub fn transition_to_ready(
    mut commands: Commands,
    localization_builder: LocalizationBuilder,
    locales_folder: Res<LocalesFolder>,
    mut next_localization_state: ResMut<NextState<LocalizationState>>,
) {
    let localization = localization_builder.build(locales_folder.deref());
    commands.insert_resource(localization);
    next_localization_state.set(LocalizationState::Ready);

    commands.remove_resource::<LocalesFolder>();
}

/// Transitions to the application when locales are ready.
pub fn transition_to_application(args: Res<Args>, mut next_app_state: ResMut<NextState<AppState>>) {
    // Transition to game mode selection screen when starting in game.
    if args.start_in_game {
        next_app_state.set(AppState::GameModeSelectionScreen);
    } else {
        next_app_state.set(AppState::MainMenu)
    }
}


/// Sets the locale.
pub fn set_locale(
    In(new_locale): In<LanguageIdentifier>,
    supported_locales: Res<SupportedLocales>,
    mut general_settings: ResMut<Persistent<GeneralSettings>>,
    mut locale: ResMut<Locale>,
    mut next_localization_state: ResMut<NextState<LocalizationState>>,
) {
    let new_locale_string = new_locale.to_string();

    if !supported_locales.contains(&new_locale) {
        log::error!("unable to set the locale to {:?} as it's not supported", new_locale_string);
        return;
    }

    log::info!("setting the locale to {:?}", new_locale_string);

    general_settings.locale = new_locale_string;
    general_settings.persist().ok();

    locale.requested = new_locale;
    next_localization_state.set(LocalizationState::Loading);
}
