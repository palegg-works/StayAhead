use super::MyTask;
use serde::{Deserialize, Serialize};

/*
 * Struct Definition
 */

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializableTask {
    pub id: i64,
    pub action: String,
    pub count_per_day: f32,
    pub unit: String,
    pub count_accum: f32,
    pub start: String,
    pub end: String,

    #[serde(default = "default_effective_dow")]
    pub effective_dow: Vec<String>,

    #[serde(default)]
    pub daily_tasks: Option<Vec<String>>,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub archive: bool,
}

fn default_effective_dow() -> Vec<String> {
    vec![
        "Monday".to_string(),
        "Tuesday".to_string(),
        "Wednesday".to_string(),
        "Thursday".to_string(),
        "Friday".to_string(),
        "Saturday".to_string(),
        "Sunday".to_string(),
    ]
}

/*
 * Conversion from MyTask to SerializableTask
 */

impl From<&MyTask> for SerializableTask {
    fn from(task: &MyTask) -> Self {
        Self {
            id: task.id,
            action: task.action.clone(),
            count_per_day: task.count_per_day,
            unit: task.unit.clone(),
            count_accum: task.count_accum,
            start: task.start.to_string(),
            end: task.end.to_string(),
            effective_dow: task.effective_dow.iter().map(|d| d.to_string()).collect(),
            daily_tasks: task.daily_tasks.clone(),
            name: task.name.clone(),
            archive: task.archive,
        }
    }
}
