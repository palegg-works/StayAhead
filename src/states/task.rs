use chrono::{NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq)]
pub struct MyTask {
    pub id: i64,
    pub action: String,
    pub count_per_day: f32,
    pub unit: String,
    pub count_accum: f32,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub effective_dow: Vec<Weekday>,
}

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
}

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
        }
    }
}

impl TryFrom<&SerializableTask> for MyTask {
    type Error = chrono::ParseError;

    fn try_from(task: &SerializableTask) -> Result<Self, Self::Error> {
        Ok(Self {
            id: task.id,
            action: task.action.clone(),
            count_per_day: task.count_per_day,
            unit: task.unit.clone(),
            count_accum: task.count_accum,
            start: NaiveDate::parse_from_str(&task.start, "%Y-%m-%d")?,
            end: NaiveDate::parse_from_str(&task.end, "%Y-%m-%d")?,
            effective_dow: task
                .effective_dow
                .iter()
                .map(|s| s.parse::<Weekday>().expect("Failed to parse day of week!"))
                .collect(),
        })
    }
}
