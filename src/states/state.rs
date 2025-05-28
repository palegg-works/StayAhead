use super::sync_mode::SyncMode;
use super::{MyTask, SerializableState};
use crate::states::{decode, encode};
use dioxus::prelude::*;

/*
 * Struct Definition
 */

#[derive(Debug, Clone)]
pub struct NoSaveAppState {
    pub sync_msg: Signal<String>,
    pub sync_mode: Signal<SyncMode>,
    pub fire_push_after_deletion: Signal<bool>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub tasks: Signal<Option<Vec<MyTask>>>,
    pub github_pat: Signal<Option<String>>,
    pub gist_id: Signal<Option<String>>,
    pub gist_file_name: Signal<Option<String>>,
}

/*
 * Conversion from SerializableState to AppState
 */

impl TryFrom<SerializableState> for AppState {
    type Error = chrono::ParseError;

    fn try_from(state: SerializableState) -> Result<Self, Self::Error> {
        if let Some(tasks) = state.tasks.clone() {
            let tasks = tasks
                .iter()
                .map(|t| {
                    MyTask::try_from(t).expect("Conversion failed from SerializableTask to MyTask")
                })
                .collect();

            Ok(AppState {
                tasks: Signal::new(Some(tasks)),
                github_pat: Signal::new(state.github_pat),
                gist_id: Signal::new(state.gist_id),
                gist_file_name: Signal::new(state.gist_file_name),
            })
        } else {
            Ok(AppState {
                tasks: Signal::new(None),
                github_pat: Signal::new(state.github_pat),
                gist_id: Signal::new(state.gist_id),
                gist_file_name: Signal::new(state.gist_file_name),
            })
        }
    }
}

/*
 * AppState sync methods
 */

impl AppState {
    pub async fn pull(&mut self) -> Result<(), String> {
        let mut tasks = self.tasks;
        let mut gist_id = self.gist_id;
        let mut github_pat = self.github_pat;
        let mut gist_file_name = self.gist_file_name;

        let content = pull_from_gist(github_pat(), gist_id(), gist_file_name())
            .await
            .map_err(|e| format!("❌ Failed to pull from Gist: {}", e.to_string()))?;

        let parsed: SerializableState = serde_json::from_str(&content)
            .map_err(|e| format!("❌ Failed to parse JSON: {}", e))?;

        let state: AppState = parsed
            .try_into()
            .map_err(|e| format!("❌ Failed to convert to AppState: {}", e))?;

        tasks.set((state.tasks)());
        gist_id.set((state.gist_id)());
        gist_file_name.set((state.gist_file_name)());

        if let Some(encoded_pat) = (state.github_pat)() {
            let decoded_pat = decode(&encoded_pat);
            github_pat.set(Some(decoded_pat));
        }

        Ok(())
    }

    pub async fn push(&mut self) -> Result<(), String> {
        let mut serializable: SerializableState = (&*self).into();

        let encrypted_pat = encode(&(self.github_pat)().unwrap());
        serializable.github_pat = Some(encrypted_pat);

        let json = serde_json::to_string_pretty(&serializable)
            .map_err(|e| format!("❌ Serialization failed: {}", e))?;

        let github_pat = (self.github_pat)();
        let gist_id = (self.gist_id)();
        let gist_file_name = (self.gist_file_name)();

        push_to_gist(github_pat, gist_id, gist_file_name, json).await
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
        let reset_time = response
            .headers()
            .get("x-ratelimit-reset")
            .and_then(|val| val.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        Err(format!(
            "Push error: {status} - {body}\n⚠️ Retry after: x-ratelimit-reset = {reset_time}"
        ))
    }
}
