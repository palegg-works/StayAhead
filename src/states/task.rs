use super::serializable_task::SerializableTask;
use chrono::{NaiveDate, Weekday};

/*
 * Struct Definition
 */

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
    pub daily_tasks: Option<Vec<String>>,
    pub name: Option<String>,
    pub archive: bool,
}

/*
 * Conversion from SerializableTask to MyTask
 */

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
            daily_tasks: task.daily_tasks.clone(),
            name: task.name.clone(),
            archive: task.archive,
        })
    }
}
