use crate::states::{decode_hex, encode_hex, xor_decrypt, xor_encrypt};
use crate::{AppState, SerializableState, SyncMode};
use dioxus::prelude::*;

const APPKEY: &str = "OBFUSCATION";

async fn push_to_gist(
    github_pat: Option<String>,
    gist_id: Option<String>,
    gist_file_name: Option<String>,
    new_content: String,
) -> Result<(), String> {
    if github_pat.is_none() || gist_id.is_none() || gist_file_name.is_none() {
        return Err("Incomplete configuration for pushing".to_string());
    }

    let github_pat = github_pat.unwrap();
    let gist_id = gist_id.unwrap();
    let gist_file_name = gist_file_name.unwrap();

    let client = reqwest::Client::new();
    let get_url = format!("https://api.github.com/gists/{}", gist_id);

    let mut files = serde_json::Map::new();
    files.insert(
        gist_file_name.to_string(),
        serde_json::json!({ "content": new_content }),
    );

    let patch_body = serde_json::json!({ "files": files });

    let response = client
        .patch(&get_url)
        .header("Authorization", format!("Bearer {}", github_pat))
        .header("User-Agent", "Stay Ahead - Palegg Works")
        .json(&patch_body)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {e}"))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!("Push error: {status} - {body}"))
    }
}

async fn pull_from_gist(
    github_pat: Option<String>,
    gist_id: Option<String>,
    gist_file_name: Option<String>,
) -> Result<String, Box<dyn std::error::Error + 'static>> {
    if github_pat.is_none() || gist_id.is_none() || gist_file_name.is_none() {
        return Err("Incomplete configuration for pulling".into());
    }

    let github_pat = github_pat.unwrap();
    let gist_id = gist_id.unwrap();
    let gist_file_name = gist_file_name.unwrap();

    let client = reqwest::Client::new();
    let get_url = format!("https://api.github.com/gists/{}", gist_id);

    let response = client
        .get(&get_url)
        .header("Authorization", format!("Bearer {}", github_pat))
        .header("User-Agent", "Stay Ahead - Palegg Works")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("GitHub API request failed: {}", response.status()).into());
    }

    let gist: serde_json::Value = response.json().await?;

    let content = gist["files"][gist_file_name]["content"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    Ok(content)
}

#[component]
pub fn Setting() -> Element {
    let app_state_push = use_context::<AppState>();

    let mut app_state = use_context::<AppState>();
    let mut sync_message = use_signal(|| "".to_string());

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
            (app_state.sync_mode).set(SyncMode::NotSynced)
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
            class: "p-6 space-y-6 max-w-xl mx-auto",
            h2 { class: "text-xl font-bold", "Gist Sync Settings" }
            h2 { class: "text-xl", {format!("Current sync status: {:?}", (app_state.sync_mode)())} }
            p { class: "text-sm text-gray-500", "Leave any field empty to stay in local-only mode." }

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
                class: "flex space-x-4",

                button {
                    class: button_css,
                    disabled: !clickable(),
                    onclick: move |_| {
                        if clickable() {
                            let mut serilizable: SerializableState = (&app_state_push).into();

                            let encrypted_pat = xor_encrypt(&(app_state_push.github_pat)().unwrap(), APPKEY);
                            let encrypted_pat = encode_hex(&encrypted_pat);
                            serilizable.github_pat = Some(encrypted_pat);

                            if let Ok(json) = serde_json::to_string_pretty(&serilizable) {
                                (app_state.sync_mode).set(SyncMode::Pushing);
                                spawn(async move {
                                    match push_to_gist(
                                        (app_state.github_pat)(),
                                        (app_state.gist_id)(),
                                        (app_state.gist_file_name)(),
                                        json).await {

                                        Ok(_) => {
                                            (app_state.sync_mode).set(SyncMode::InSync);
                                            sync_message.set("✅ Push was successful!".to_string());
                                        },
                                        Err(e) => {
                                            (app_state.sync_mode).set(SyncMode::Failed);
                                            sync_message.set(format!("❌ {}", e));
                                        },
                                    }
                                });
                            }
                        }
                    },
                    "Push: upload local data"
                }

                button {
                    class: button_css,
                    disabled: !clickable(),
                    onclick: move |_| {
                        if clickable() {
                            (app_state.sync_mode).set(SyncMode::Pulling);
                            spawn(async move {
                                match pull_from_gist(
                                    (app_state.github_pat)(),
                                    (app_state.gist_id)(),
                                    (app_state.gist_file_name)()).await {

                                    Ok(content) => {
                                        if let Ok(parsed) = serde_json::from_str::<SerializableState>(&content) {
                                            if let Ok(state) = TryInto::<AppState>::try_into(parsed) {
                                                app_state.tasks.set((state.tasks)());
                                                app_state.gist_id.set((state.gist_id)());
                                                app_state.gist_file_name.set((state.gist_file_name)());

                                                let decoded_pat = decode_hex(&(state.github_pat)().unwrap());
                                                let decoded_pat = xor_decrypt(&decoded_pat, APPKEY);
                                                app_state.github_pat.set(Some(decoded_pat));

                                                app_state.sync_mode.set(SyncMode::InSync);
                                                sync_message.set("✅ Pull was successful!".to_string());
                                                return;
                                            }
                                        }

                                        (app_state.sync_mode).set(SyncMode::Failed);
                                        sync_message.set("❌ Failed at parsing pulled data!".to_string());
                                    },
                                    Err(e) => {
                                        (app_state.sync_mode).set(SyncMode::Failed);
                                        sync_message.set(format!("❌ {}", e));
                                    },
                                }
                            });
                        }
                    },
                    "Pull: restore from cloud data"
                }
            }

            div {
                label { "Sync Message" }
                textarea {
                    class: "mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm h-32",
                    value: sync_message,
                    oninput: move |evt| sync_message.set(evt.value()),
                }
            }

        }
    }
}
