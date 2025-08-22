mod components;
mod states;

use core::sync;

use components::Route;
use dioxus::prelude::*;
use states::{AppState, NoSaveAppState, SerializableState, SyncMode};

const FAVICON: Asset = asset!("/assets/icons/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind_output.css");

fn main() {
    console_log::init_with_level(log::Level::Info).expect("error initializing log");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    use_effect(move || {
        document::eval(
            r#"
            document.body.classList.add('loaded');
            "#
        );
    });
    
    let no_save_app_state = use_context_provider(|| NoSaveAppState {
        sync_msg: Signal::new("".to_string()),
        sync_mode: Signal::new(SyncMode::NotSynced),
        fire_push_after_deletion: Signal::new(false),
    });

    let mut sync_msg = no_save_app_state.sync_msg;
    let mut sync_mode = no_save_app_state.sync_mode;

    let mut app_state = use_context_provider(|| {
        let mut app_state = match SerializableState::load() {
            Ok(state) => match TryInto::<AppState>::try_into(state) {
                Ok(state) => state,
                _ => AppState {
                    tasks: Signal::new(None),
                    github_pat: Signal::new(None),
                    gist_id: Signal::new(None),
                    gist_file_name: Signal::new(None),
                },
            },
            Err(_) => AppState {
                tasks: Signal::new(None),
                github_pat: Signal::new(None),
                gist_id: Signal::new(None),
                gist_file_name: Signal::new(None),
            },
        };

        // Trigger a initial pull
        let mut app_state_for_pull = app_state.clone();

        spawn({
            sync_mode.set(SyncMode::Pulling);

            async move {
                match app_state_for_pull.pull().await {
                    Ok(_) => {
                        sync_msg.set("✅ Initial pull was successful!".to_string());
                        sync_mode.set(SyncMode::InSync);
                    }
                    Err(e) => {
                        sync_msg.set("⚠️ Initial pull failed. Not synced!".to_string());
                        sync_mode.set(SyncMode::NotSynced);
                    }
                }
            }
        });

        app_state
    });

    use_effect(move || {
        app_state.save();
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
