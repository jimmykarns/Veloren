#![deny(unsafe_code)]
#![recursion_limit = "2048"]

use veloren_voxygen::{
    audio::{self, AudioFrontend},
    i18n::{self, i18n_asset_key, VoxygenLocalization},
    logging,
    menu::main::MainMenuState,
    meta::Meta,
    render::Renderer,
    settings::Settings,
    window::Window,
    Direction, GlobalState, PlayState, PlayStateResult,
};

use common::{
    assets::{load, load_expect, load_watched, watch},
    clock::Clock,
};
use log::{debug, error, warn};
use std::{mem, panic, str::FromStr};

fn main() {
    // Initialize logging.
    let term_log_level = std::env::var_os("VOXYGEN_LOG")
        .and_then(|env| env.to_str().map(|s| s.to_owned()))
        .and_then(|s| log::LevelFilter::from_str(&s).ok())
        .unwrap_or(log::LevelFilter::Warn);

    let file_log_level = std::env::var_os("VOXYGEN_FILE_LOG")
        .and_then(|env| env.to_str().map(|s| s.to_owned()))
        .and_then(|s| log::LevelFilter::from_str(&s).ok())
        .unwrap_or(log::LevelFilter::Debug);

    // Load the settings
    // Note: This won't log anything due to it being called before
    // ``logging::init``.       The issue is we need to read a setting to decide
    // whether we create a log file or not.
    let mut settings = Settings::load();
    // Save settings to add new fields or create the file if it is not already there
    if let Err(err) = settings.save_to_file() {
        panic!("Failed to save settings: {:?}", err);
    }

    logging::init(&settings, term_log_level, file_log_level);

    // Set up panic handler to relay swish panic messages to the user
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let panic_info_payload = panic_info.payload();
        let payload_string = panic_info_payload.downcast_ref::<String>();
        let reason = match payload_string {
            Some(s) => &s,
            None => {
                let payload_str = panic_info_payload.downcast_ref::<&str>();
                match payload_str {
                    Some(st) => st,
                    None => "Payload is not a string",
                }
            },
        };
        let msg = format!(
            "A critical error has occurred and Voxygen has been forced to \
            terminate in an unusual manner. Details about the error can be \
            found below.\n\
            \n\
            > What should I do?\n\
            \n\
            We need your help to fix this! You can help by contacting us and \
            reporting this problem. To do this, open an issue on the Veloren \
            issue tracker:\n\
            \n\
            https://www.gitlab.com/veloren/veloren/issues/new\n\
            \n\
            If you're on the Veloren community Discord server, we'd be \
            grateful if you could also post a message in the #support channel.
            \n\
            > What should I include?\n\
            \n\
            The error information below will be useful in finding and fixing \
            the problem. Please include as much information about your setup \
            and the events that led up to the panic as possible.
            \n\
            Voxygen has logged information about the problem (including this \
            message) to the file {:#?}. Please include the contents of this \
            file in your bug report.
            \n\
            > Error information\n\
            \n\
            The information below is intended for developers and testers.\n\
            \n\
            Panic Payload: {:?}\n\
            PanicInfo: {}",
            // TODO: Verify that this works
            Settings::get_settings_path()
                .join("voxygen-<date>.log")
                .display(),
            reason,
            panic_info,
        );

        error!(
            "VOXYGEN HAS PANICKED\n\n{}\n\nBacktrace:\n{:?}",
            msg,
            backtrace::Backtrace::new(),
        );

        #[cfg(feature = "msgbox")]
        {
            #[cfg(target_os = "macos")]
            dispatch::Queue::main()
                .sync(|| msgbox::create("Voxygen has panicked", &msg, msgbox::IconType::Error));
            #[cfg(not(target_os = "macos"))]
            msgbox::create("Voxygen has panicked", &msg, msgbox::IconType::Error);
        }

        default_hook(panic_info);
    }));

    // Create window
    let (window, event_loop) = Window::new(&settings).expect("Failed to create window!");

    // Setup audio
    let mut audio = if settings.audio.audio_on {
        AudioFrontend::new(
            settings
                .audio
                .audio_device
                .as_ref()
                .map_or_else(|| audio::get_default_device(), |d| d.to_string()),
            settings.audio.max_sfx_channels,
        )
    } else {
        AudioFrontend::no_audio()
    };

    audio.set_music_volume(settings.audio.music_volume);
    audio.set_sfx_volume(settings.audio.sfx_volume);

    // Load metadata
    let meta = Meta::load();

    // Try to load the localization and log missing entries
    // Keep a watcher on the language TODO: avoid need for this
    let mut localization_watcher = watch::ReloadIndicator::new();
    let localized_strings = load_watched::<VoxygenLocalization>(
        &i18n_asset_key(&settings.language.selected_language),
        &mut localization_watcher,
    )
    .unwrap_or_else(|error| {
        warn!(
            "Impossible to load {} language: changing to the default language (English) instead. \
             Source error: {:?}",
            &settings.language.selected_language, error
        );
        settings.language.selected_language = i18n::REFERENCE_LANG.to_owned();
        load_watched::<VoxygenLocalization>(
            &i18n_asset_key(&settings.language.selected_language),
            &mut localization_watcher,
        )
        .unwrap()
    });
    localized_strings.log_missing_entries();

    let global_state = GlobalState {
        audio,
        window,
        settings,
        clock: Clock::start(),
        meta,
        info_message: None,
        #[cfg(feature = "singleplayer")]
        singleplayer: None,
        localization_watcher,
    };

    run::run(global_state, event_loop);
}
