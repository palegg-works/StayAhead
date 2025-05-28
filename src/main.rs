mod components;
mod states;

use components::Route;
use dioxus::prelude::*;
use states::{AppState, LoadError, SerializableState, SyncMode};

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
        Err(LoadError::FileNotFound(_)) => AppState {
            tasks: Signal::new(None),
            sync_mode: Signal::new(SyncMode::NotSynced),
            github_pat: Signal::new(None),
            gist_id: Signal::new(None),
            gist_file_name: Signal::new(None),
        },
        Err(LoadError::InvalidJson(_)) => AppState {
            tasks: Signal::new(None),
            sync_mode: Signal::new(SyncMode::NotSynced),
            github_pat: Signal::new(None),
            gist_id: Signal::new(None),
            gist_file_name: Signal::new(None),
        },
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
