use crate::{AppState, NoSaveAppState, SerializableState, SyncMode};
use dioxus::prelude::*;

const APPKEY: &str = "OBFUSCATION";

#[component]
pub fn Setting() -> Element {
    let mut app_state_push = use_context::<AppState>();
    let mut app_state = use_context::<AppState>();

    let no_save_app_state = use_context::<NoSaveAppState>();
    let mut sync_msg = no_save_app_state.sync_msg;
    let mut sync_mode = no_save_app_state.sync_mode;

    let clickable = use_memo(move || {
        if let Some(github_pat) = (app_state.github_pat)() {
            if let Some(gist_id) = (app_state.gist_id)() {
                if let Some(gist_file_name) = (app_state.gist_file_name)() {
                    if !github_pat.is_empty() && !gist_id.is_empty() && !gist_file_name.is_empty() {
                        return true;
                    }
                }
            }
        }

        false
    });

    use_effect(move || {
        if !clickable() {
            sync_mode.set(SyncMode::NotSynced)
        }
    });

    let button_css = use_memo(move || {
        format!(
            "font-semibold py-2 px-4 rounded transition-colors duration-300 {}",
            if clickable() {
                "bg-blue-600 hover:bg-blue-700 text-white cursor-pointer"
            } else {
                "bg-gray-300 text-gray-500 cursor-not-allowed"
            }
        )
    });

    rsx! {
        div {
            class: "p-6 max-w-xl mx-auto space-y-6 bg-white rounded-xl shadow",

            h2 { class: "text-xl font-bold", "Sync Settings" }
            p { class: "text-sm text-gray-500", "⚠️ Sync functionality is currently in beta. Use with caution." }

            div {
                label { "GitHub Gist ID" }
                input {
                    class: "w-full border rounded p-2",
                    value: app_state.gist_id,
                    oninput: move |evt| {
                        let user_input = evt.value();
                        if user_input.is_empty() {
                            (app_state.gist_id).set(None);
                        } else {
                            (app_state.gist_id).set(Some(user_input));
                        }
                    },
                }
            }

            div {
                label { "Gist File Name" }
                input {
                    class: "w-full border rounded p-2",
                    value: app_state.gist_file_name,
                    oninput: move |evt| {
                        let user_input = evt.value();
                        if user_input.is_empty() {
                            (app_state.gist_file_name).set(None);
                        } else {
                            (app_state.gist_file_name).set(Some(user_input));
                        }
                    },
                }
            }

            div {
                label { "GitHub Personal Access Token (Classic)" }
                input {
                    r#type: "password",
                    class: "w-full border rounded p-2",
                    value: app_state.github_pat,
                    oninput: move |evt| {
                        let user_input = evt.value();
                        if user_input.is_empty() {
                            (app_state.github_pat).set(None)
                        } else {
                            (app_state.github_pat).set(Some(user_input))
                        }
                    },
                }
            }

            div {
                class: "flex space-x-4 justify-center",

                button {
                    class: button_css,
                    disabled: !clickable(),
                    onclick: move |_| {
                        if clickable() {
                            spawn({
                                sync_mode.set(SyncMode::Pushing);
                                async move {
                                    let mut app_state = use_context::<AppState>();
                                    match app_state.push().await {
                                        Ok(_) => {
                                            sync_msg.set("✅ Manual push was successful!".to_string());
                                            sync_mode.set(SyncMode::InSync);
                                        },
                                        Err(e) => {
                                            sync_msg.set(format!("⚠️ Manual push failed: {}", e));
                                            sync_mode.set(SyncMode::NotSynced);
                                        },
                                    }
                                }
                            });
                        }
                    },
                    "Push: upload local data"
                }

                button {
                    class: button_css,
                    disabled: !clickable(),
                    onclick: move |_| {
                        if clickable() {
                            spawn({
                                sync_mode.set(SyncMode::Pulling);
                                async move {
                                    let mut app_state = use_context::<AppState>();
                                    match app_state.pull().await {
                                        Ok(_) => {
                                            sync_msg.set("✅ Manual pull was successful!".to_string());
                                            sync_mode.set(SyncMode::InSync);
                                        },
                                        Err(e) => {
                                            sync_msg.set(format!("⚠️ Manual pull failed: {}", e));
                                            sync_mode.set(SyncMode::NotSynced);
                                        },
                                    }
                                }
                            });
                        }
                    },
                    "Pull: restore from cloud data"
                }
            }

            div {
                h2 { class: "text-xl", { format!("Current sync status: {:?}", sync_mode()) } }

                label { "Additional sync message:" }
                textarea {
                    class: "mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm h-32",
                    value: sync_msg,
                    readonly: true,
                }
            }

        }
    }
}
