use crate::states::MyTask;
use crate::{AppState, NoSaveAppState, SyncMode};
use chrono::{Datelike, Local, NaiveDate, Weekday};
use dioxus::prelude::*;
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

static ALL_WEEKDAYS: [Weekday; 7] = [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

#[derive(PartialEq, Clone, Copy, EnumIter)]
enum TaskCreationMode {
    SameActEveryDay,
    SpecificActEachDay,
}

impl std::fmt::Display for TaskCreationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TaskCreationMode::SameActEveryDay => {
                write!(f, "Habit Builder: Complete the same activity every day!")
            }
            TaskCreationMode::SpecificActEachDay => {
                write!(f, "Goal Achiever: Define specific activities every day!")
            }
        }
    }
}

pub fn calculate_completion_date(
    n_tasks: usize,
    start_date: NaiveDate,
    effective_dow: Vec<Weekday>,
) -> Option<NaiveDate> {
    // If no tasks, it's already complete on the start date (or before)
    if n_tasks == 0 {
        return Some(start_date);
    }
    // If no effective days, tasks can never be completed
    if effective_dow.is_empty() {
        return None;
    }

    // Use a HashSet for efficient O(1) lookup of effective days
    let effective_days_set: HashSet<Weekday> = effective_dow.iter().copied().collect();

    let mut current_date = start_date;
    let mut tasks_completed_count = 0;

    // Loop until all tasks are completed
    while tasks_completed_count < n_tasks {
        // Check if the current day of the week is one of the effective days
        if effective_days_set.contains(&current_date.weekday()) {
            tasks_completed_count += 1;
        }

        // If tasks are all completed on the current_date, break before incrementing date
        if tasks_completed_count == n_tasks {
            break;
        }

        // Move to the next day
        current_date += chrono::Duration::days(1);
    }

    Some(current_date)
}

#[component]
pub fn TaskCreate() -> Element {
    let no_save_app_state = use_context::<NoSaveAppState>();
    let mut sync_msg = no_save_app_state.sync_msg;
    let mut sync_mode = no_save_app_state.sync_mode;
    let mut fire_push = use_signal(|| false);

    let mut app_state = use_context::<AppState>();

    let today_str = Local::now().date_naive().to_string();
    let tomorrow_str = (Local::now().date_naive() + chrono::Duration::days(1)).to_string();
    let one_week_later_str = (Local::now().date_naive() + chrono::Duration::days(7)).to_string();

    let mut action = use_signal(|| "".to_string());
    let mut count_per_day = use_signal(|| 1.0_f32);
    let mut unit = use_signal(|| "".to_string());
    let mut start = use_signal(|| today_str.to_string());
    let mut end = use_signal(|| one_week_later_str.to_string());
    let mut submit_return_msg = use_signal(|| "".to_string());
    let mut selected_dow = use_signal(|| ALL_WEEKDAYS.to_vec());

    let mut raw_acts = use_signal(|| "".to_string());
    let mut customized_task_name = use_signal(|| "".to_string());

    let mut selected_creation_mode = use_signal(|| TaskCreationMode::SameActEveryDay);

    let enable_submit_same_mode = use_memo(move || {
        !action().is_empty()
            && count_per_day() > 0.0_f32
            && !unit().is_empty()
            && !selected_dow.is_empty()
    });

    let enable_submit_spec_mode =
        use_memo(move || !raw_acts().is_empty() && !customized_task_name().is_empty());

    use_effect(move || {
        let _ = selected_creation_mode.read();
        submit_return_msg.set("".to_string());
    });

    use_effect(move || {
        if fire_push() {
            spawn({
                sync_mode.set(SyncMode::Pushing);
                async move {
                    let mut app_state = use_context::<AppState>();
                    match app_state.push().await {
                        Ok(_) => {
                            sync_msg.set(
                                "✅ Automatic push was successful after creating a task!"
                                    .to_string(),
                            );
                            sync_mode.set(SyncMode::InSync);
                        }
                        Err(e) => {
                            sync_msg.set(format!(
                                "⚠️ Automatic push failed after creating a task: {}",
                                e
                            ));
                            sync_mode.set(SyncMode::NotSynced);
                        }
                    }
                }
            });
        }
    });

    rsx! {
        div {
            class: "p-6 max-w-xl mx-auto space-y-4 bg-white rounded-xl shadow",

            // Mode selection boxes
            div {
                class: "grid grid-cols-2 gap-4 mb-6",
                {
                    TaskCreationMode::iter().map(|current_mode| {
                        let is_selected = selected_creation_mode() == current_mode;
                        rsx! {
                            div {
                                class: format!(
                                           "cursor-pointer p-4 rounded-lg text-center font-semibold transition-colors duration-200 {}",
                                           if is_selected {
                                               "bg-blue-400 text-white shadow-md"
                                           } else {
                                               "bg-gray-200 text-gray-700 hover:bg-gray-300"
                                           }
                                       ),
                                onclick: move |_| selected_creation_mode.set(current_mode),
                                "{current_mode}",
                            }
                        }
                    })
                }
            }

            // Collect information
            match selected_creation_mode() {
                TaskCreationMode::SameActEveryDay => rsx! {
                    div {
                        class: "space-y-2 text-gray-800",
                        p { class: "text-lg font-bold", "😎 Tell me what you want to do every day!" }
                        p { "Below are some examples:" }
                        ul { class: "list-disc list-inside space-y-1 text-sm",
                        li { "I want to (read) (10) (minutes) per day" }
                        li { "I want to (write) (5) (paragraphs) per day" }
                        }
                    }

                    div {
                        class: "text-sm text-gray-800 font-medium",
                        "Every day, I want to"
                    }

                    div {
                        class: "flex gap-x-4 items-center",

                        div {
                            input {
                                r#type: "text",
                                class: "w-full border border-gray-300 rounded-md shadow-sm p-2",
                                placeholder: "Action",
                                value: "{action}",
                                oninput: move |e| action.set(e.value())
                            }
                        }

                        div {
                            class: "w-1/6",
                            input {
                                r#type: "number",
                                step: "any",
                                min: "0",
                                class: "w-full border border-gray-300 rounded-md shadow-sm p-2",
                                placeholder: "Count",
                                value: "{count_per_day}",
                                oninput: move |e| {
                                    if let Ok(num) = e.value().parse::<f32>() {
                                        count_per_day.set(num)
                                    } else {
                                        count_per_day.set(0.0_f32)
                                    }
                                }
                            }
                        }

                        div {
                            class: "w-1/3",
                            input {
                                r#type: "text",
                                class: "w-full border border-gray-300 rounded-md shadow-sm p-2",
                                placeholder: "Unit",
                                autocomplete: "off",
                                spellcheck: "false",
                                value: "{unit}",
                                oninput: move |e| unit.set(e.value().clone())
                            }
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Start Date" }
                        input {
                            r#type: "date",
                            class: "mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2",
                            value: "{start}",
                            oninput: move |e| start.set(e.value().clone())
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700", "End Date" }
                        input {
                            r#type: "date",
                            min: "{tomorrow_str}",
                            class: "mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2",
                            value: "{end}",
                            oninput: move |e| end.set(e.value().clone())
                        }
                    }

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700",
                            "This task will be effective on these days:",
                        },

                        div {
                            class: "flex flex-wrap gap-4 mt-2",
                            {
                                ALL_WEEKDAYS.iter().map(|d| {
                                    let short_label = d.to_string();
                                    let selected = selected_dow.read().contains(d);

                                    rsx! {
                                        label {
                                            class: "inline-flex items-center space-x-2 cursor-pointer select-none",
                                            input {
                                                r#type: "checkbox",
                                                checked: selected,
                                                onchange: move |evt| {
                                                    let is_now_checked = evt.value() == "true";
                                                    if is_now_checked {
                                                        if !selected_dow.read().contains(d) {
                                                            selected_dow.write().push(d.to_owned());
                                                        }
                                                    } else if selected_dow.read().contains(d) {
                                                        selected_dow.write().retain(|e| e != d);
                                                    }
                                                }
                                            },
                                            span { "{short_label}" }
                                        }
                                    }
                                })
                            }
                        }
                    }

                    div {
                        class: "pt-4 flex items-center gap-x-4",
                        button {
                            disabled: "{!enable_submit_same_mode()}",
                            class: format_args!(
                                "font-semibold py-2 px-4 rounded transition-colors duration-300 {}",
                                if enable_submit_same_mode() {
                                    "bg-blue-600 hover:bg-blue-700 text-white cursor-pointer"
                                } else {
                                    "bg-gray-300 text-gray-500 cursor-not-allowed"
                                }
                            ),
                            onclick: move |_| {
                                if let (Ok(start_date), Ok(end_date)) = (
                                    NaiveDate::parse_from_str(&start(), "%Y-%m-%d"),
                                    NaiveDate::parse_from_str(&end(), "%Y-%m-%d")
                                ) {
                                    selected_dow.write().sort_by_key(|d| d.num_days_from_monday());

                                    let task = MyTask {
                                        id: chrono::Utc::now().timestamp_millis(),
                                        action: action().clone(),
                                        count_per_day: count_per_day(),
                                        unit: unit().clone(),
                                        count_accum: 0.0_f32,
                                        start: start_date,
                                        end: end_date,
                                        effective_dow: selected_dow(),
                                        daily_tasks: None,
                                        name: None,
                                        archive: false,
                                    };

                                    app_state.tasks.write().as_mut().unwrap().insert(task.id, task);

                                    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                                    submit_return_msg.set(format!("✅ Created a task ({})", now));
                                    fire_push.set(true);
                                } else {
                                    submit_return_msg.set("❌ Task creation failed! Please check dates!".to_string());
                                }
                            },
                            "Submit",
                        },

                        if !submit_return_msg().is_empty() {
                            div {
                                class: "text-sm text-gray-700",
                                "{submit_return_msg}"
                            }
                        }
                    }
                },
                TaskCreationMode::SpecificActEachDay => rsx!{
                    div {
                        class: "space-y-2 text-gray-800",
                        p { class: "text-lg font-bold", "😎 Let's define your everyday activities" }
                        p { "You will need to think hard about this one: what do you want to do on each day?" }
                        p { "It is recommended to compose your list of daily tasks in a text file if you have a challenge task, one line per day. Then you can copy and paste the content below." }
                    }

                    div {
                        class: "text-sm text-gray-800 font-medium",
                        "Give this challenge a unique name"
                    }

                    div {
                        input {
                            r#type: "text",
                            class: "w-full border border-gray-300 rounded-md shadow-sm p-2",
                            placeholder: "Inspiring name for this challenge",
                            value: "{customized_task_name}",
                            oninput: move |e| customized_task_name.set(e.value())
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Start Date" }
                        input {
                            r#type: "date",
                            class: "mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2",
                            value: "{start}",
                            oninput: move |e| start.set(e.value().clone())
                        }
                    }

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700",
                            "Daily Activities (One line is one day):"
                        }
                        textarea {
                            class: "mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2 h-32", // Added h-32 for height
                            placeholder: "e.g.,\nRead 10 pages <- day 1\nWrite 500 words <- day 2\nExercise 30 mins <- day 3",
                            value: "{raw_acts}",
                            oninput: move |e| raw_acts.set(e.value()),
                        }
                    }

                    div {
                        label {
                            class: "block text-sm font-medium text-gray-700",
                            "This task will be effective on these days:",
                        },

                        div {
                            class: "flex flex-wrap gap-4 mt-2",
                            {
                                ALL_WEEKDAYS.iter().map(|d| {
                                    let short_label = d.to_string();
                                    let selected = selected_dow.read().contains(d);

                                    rsx! {
                                        label {
                                            class: "inline-flex items-center space-x-2 cursor-pointer select-none",
                                            input {
                                                r#type: "checkbox",
                                                checked: selected,
                                                onchange: move |evt| {
                                                    let is_now_checked = evt.value() == "true";
                                                    if is_now_checked {
                                                        if !selected_dow.read().contains(d) {
                                                            selected_dow.write().push(d.to_owned());
                                                        }
                                                    } else if selected_dow.read().contains(d) {
                                                        selected_dow.write().retain(|e| e != d);
                                                    }
                                                }
                                            },
                                            span { "{short_label}" }
                                        }
                                    }
                                })
                            }
                        }
                    }

                    div {
                        class: "pt-4 flex items-center gap-x-4",
                        button {
                            disabled: "{!enable_submit_spec_mode()}",
                            class: format_args!(
                                "font-semibold py-2 px-4 rounded transition-colors duration-300 {}",
                                if enable_submit_spec_mode() {
                                    "bg-blue-600 hover:bg-blue-700 text-white cursor-pointer"
                                } else {
                                    "bg-gray-300 text-gray-500 cursor-not-allowed"
                                }
                            ),
                            onclick: move |_| {
                                if let Ok(start_date) = NaiveDate::parse_from_str(&start(), "%Y-%m-%d") {

                                    selected_dow.write().sort_by_key(|d| d.num_days_from_monday());

                                    let fmt_acts = raw_acts().lines().map(|line| line.trim().to_string()).filter(|line| !line.is_empty()).collect::<Vec<String>>();

                                    if let Some(end_date) = calculate_completion_date(fmt_acts.len(), start_date, selected_dow()) {
                                        let task = MyTask {
                                            id: chrono::Utc::now().timestamp_millis(),
                                            action: "Complete".to_string(),
                                            count_per_day: 1.0_f32,
                                            unit: "line of daily activities".to_string(),
                                            count_accum: 0.0_f32,
                                            start: start_date,
                                            end: end_date,
                                            effective_dow: selected_dow(),
                                            daily_tasks: Some(fmt_acts),
                                            name: Some(customized_task_name()),
                                            archive: false,
                                        };

                                        app_state.tasks.write().as_mut().unwrap().insert(task.id, task);

                                        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                                        submit_return_msg.set(format!("✅ Created a task ({})", now));
                                        fire_push.set(true);
                                    } else {
                                        submit_return_msg.set("❌ Task creation failed! Please check dates!".to_string());
                                    }
                                } else {
                                    submit_return_msg.set("❌ Task creation failed! Please check dates!".to_string());
                                }
                            },
                            "Submit",
                        },

                        if !submit_return_msg().is_empty() {
                            div {
                                class: "text-sm text-gray-700",
                                "{submit_return_msg}"
                            }
                        }
                    }
                },
            }
        }
    }
}
