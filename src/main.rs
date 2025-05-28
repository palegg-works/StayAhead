mod components;
mod states;

use components::Route;
use dioxus::prelude::*;
use states::{AppState, NoSaveAppState, SerializableState, SyncMode};

const FAVICON: Asset = asset!("/assets/icons/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind_output.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let app_state = use_context_provider(|| match SerializableState::load() {
        Ok(state) => match TryInto::<AppState>::try_into(state) {
            Ok(state) => state,
            _ => AppState {
                tasks: Signal::new(None),
                sync_mode: Signal::new(SyncMode::NotSynced),
                github_pat: Signal::new(None),
                gist_id: Signal::new(None),
                gist_file_name: Signal::new(None),
            },
        },
        Err(_) => AppState {
            tasks: Signal::new(None),
            sync_mode: Signal::new(SyncMode::NotSynced),
            github_pat: Signal::new(None),
            gist_id: Signal::new(None),
            gist_file_name: Signal::new(None),
        },
    });

    let _ = use_context_provider(|| NoSaveAppState {
        sync_msg: Signal::new("".to_string()),
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
