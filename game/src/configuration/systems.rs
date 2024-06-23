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
    supported_locales: Res<SupportedLocales>,
    locale_assets: Res<LocaleAssets>,
    locale: Res<Locale>,
) {
    let mut handles = Vec::with_capacity(supported_locales.len() * (1 + locale_assets.len()));
    for supported_locale in supported_locales.iter() {
        if supported_locale != &locale.requested
            && Some(supported_locale) != locale.default.as_ref()
        {
            continue;
        }


        let path = format!("locales/{}/main.ftl.yml", supported_locale);
        log::info!("loading asset {}", path);
        handles.push(asset_server.load::<BundleAsset>(path).untyped());

        for asset in locale_assets.iter() {
            let path = format!("locales/{}/{}", supported_locale, asset);
            log::info!("loading asset {}", path);
            handles.push(asset_server.load::<ResourceAsset>(path).untyped());
        }
    }
    commands.insert_resource(LocaleAssetHandles(handles));
}

/// Loads locales folder.
pub fn load_locales_folder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loaded_folders: ResMut<Assets<LoadedFolder>>,
    locale_asset_handles: Res<LocaleAssetHandles>,
) {
    let mut handles = Vec::with_capacity(locale_asset_handles.len());
    for handle in locale_asset_handles.iter() {
        if asset_server.get_load_state(handle) == Some(LoadState::Loaded) {
            handles.push(handle.clone());
        }
    }

    commands.insert_resource(LocalesFolder(
        if handles.is_empty() { None } else { Some(loaded_folders.add(LoadedFolder { handles })) },
    ));
    commands.remove_resource::<LocaleAssetHandles>();
}

/// Transitions to the ready when locales folder is loaded.
pub fn transition_to_ready(
    mut commands: Commands,
    localization_builder: LocalizationBuilder,
    locales_folder: Res<LocalesFolder>,
    mut next_localization_state: ResMut<NextState<LocalizationState>>,
) {
    let handle = match &locales_folder.0 {
        Some(handle) => handle,
        None => &Handle::<LoadedFolder>::default(),
    };

    let localization = localization_builder.build(handle);
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
    mut general_settings: ResMut<Persistent<GeneralSettings>>,
    mut locale: ResMut<Locale>,
    mut next_localization_state: ResMut<NextState<LocalizationState>>,
) {
    let new_locale_string = new_locale.to_string();
    log::info!("setting the locale to {:?}", new_locale_string);

    general_settings.locale = new_locale_string;
    general_settings.persist().ok();

    locale.requested = new_locale;
    next_localization_state.set(LocalizationState::Loading);
}
