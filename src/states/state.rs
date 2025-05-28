use super::sync_mode::SyncMode;
use super::{MyTask, SerializableState};
use dioxus::prelude::*;

/*
 * Struct Definition
 */

#[derive(Debug, Clone)]
pub struct NoSaveAppState {
    pub sync_msg: Signal<String>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub tasks: Signal<Option<Vec<MyTask>>>,
    pub sync_mode: Signal<SyncMode>,
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
                sync_mode: Signal::new(SyncMode::NotSynced),
            })
        } else {
            Ok(AppState {
                tasks: Signal::new(None),
                github_pat: Signal::new(state.github_pat),
                gist_id: Signal::new(state.gist_id),
                gist_file_name: Signal::new(state.gist_file_name),
                sync_mode: Signal::new(SyncMode::NotSynced),
            })
        }
    }
}
