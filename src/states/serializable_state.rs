use super::serializable_task::SerializableTask;
use super::AppState;
use serde::{Deserialize, Serialize};

/*
 * Struct Definition
 */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableState {
    pub tasks: Option<Vec<SerializableTask>>,

    #[serde(default)]
    pub github_pat: Option<String>,

    #[serde(default)]
    pub gist_id: Option<String>,

    #[serde(default)]
    pub gist_file_name: Option<String>,
}

/*
 * Conversion from AppState to SerializableState
 */

impl From<&AppState> for SerializableState {
    fn from(state: &AppState) -> Self {
        let tasks = (state.tasks)();
        if let Some(tasks) = tasks {
            let tasks: Vec<SerializableTask> = tasks.values().map(SerializableTask::from).collect();
            SerializableState {
                tasks: Some(tasks),
                github_pat: (state.github_pat)(),
                gist_id: (state.gist_id)(),
                gist_file_name: (state.gist_file_name)(),
            }
        } else {
            SerializableState {
                tasks: None,
                github_pat: (state.github_pat)(),
                gist_id: (state.gist_id)(),
                gist_file_name: (state.gist_file_name)(),
            }
        }
    }
}
