use super::routes::Route;
use crate::states::AppState;
use chrono::{Datelike, Duration, Local, NaiveDate, Timelike};
use dioxus::prelude::*;
use std::cmp::Ordering;

fn generate_date_range(
    start: NaiveDate,
    end: NaiveDate,
    effective_dow: Vec<chrono::Weekday>,
) -> Vec<NaiveDate> {
    let mut dates = vec![];
    let mut current = start;
    while current <= end {
        if effective_dow.contains(&current.weekday()) {
            dates.push(current);
        }
        current += Duration::days(1);
    }
    dates
}

fn fill_ratio_parallel_universe(date: NaiveDate, today: NaiveDate) -> f32 {
    match date.cmp(&today) {
        Ordering::Less => 1.0,
        Ordering::Equal => {
            let now = Local::now().time();
            now.num_seconds_from_midnight() as f32 / 86400.0
        }
        Ordering::Greater => 0.0,
    }
}

fn fill_ratio_user_universe(index: usize, count_per_day: f32, count_accum: f32) -> f32 {
    let full_boxes = (count_accum / count_per_day).floor() as usize;
    let remaining = count_accum % count_per_day;

    match index.cmp(&full_boxes) {
        Ordering::Less => 1.0,
        Ordering::Equal => remaining / count_per_day,
        Ordering::Greater => 0.0,
    }
}

#[component]
pub fn TaskVisual(id: i64) -> Element {
    let mut app_state = use_context::<AppState>();
    let navigator = use_navigator();

    let task = (app_state.tasks)().and_then(|tasks| tasks.iter().find(|t| t.id == id).cloned());

    if task.is_none() {
        return rsx! { p { "Task not found." } };
    }

    let task = task.unwrap();
    let dates = generate_date_range(task.start, task.end, task.effective_dow);
    let today = Local::now().date_naive();

    let total_days = dates.len();
    let days_passed_inclusive = dates.iter().filter(|&&d| d <= today).count();

    let user_accomplished = task.count_accum;
    let user_remaining = (total_days as f32 * task.count_per_day) - user_accomplished;

    let parallel_accomplished = days_passed_inclusive as f32 * task.count_per_day;
    let parallel_remaining = (total_days as f32 * task.count_per_day) - parallel_accomplished;

    let mut show_details = use_signal(|| true);
    let has_details = use_signal(|| task.daily_tasks.is_some());
    let mut n_clicks_on_remove = use_signal(|| 0_i64);

    rsx! {
        div {
            class: "p-6 max-w-5xl mx-auto space-y-6",

            h2 { class: "text-xl font-bold text-center", "You told me that you want to ..."}
            h2 { class: "text-xl font-bold text-center", "\"{task.action} {task.count_per_day} {task.unit} every day\"" }
            h2 { class: "text-xl font-bold text-center", "Let's see how well you have done! 😎"}

            div { class: "text-center font-semibold text-green-700", "🚀 Let's Go!" }

            // A single grid container for the headers
            div {
                class: "grid grid-cols-3 gap-1 mb-4", // Added mb-4 for spacing below headers
                div { class: "flex flex-col items-center", p { class: "font-medium text-blue-800", "🌌 Parallel Universe" } }

                if has_details() {
                    div {
                        class: format!(
                                   "cursor-pointer rounded-lg text-center font-semibold transition-colors duration-200 {}",
                                   if show_details() {
                                       "bg-blue-600 text-white shadow-md"
                                   } else {
                                       "bg-gray-200 text-gray-700 hover:bg-gray-300"
                                   }
                               ),
                               onclick: move |_| show_details.set(!show_details()),
                               "📅 Timeline",
                    }
                } else {
                    div { class: "flex flex-col items-center", p { class: "font-medium text-gray-500", "📅 Timeline" } }
                }

                div { class: "flex flex-col items-center", p { class: "font-medium text-purple-800", "🪐 Your Universe" } }
            }

            // A single grid for each available date
            {
                dates.iter().enumerate().map(|(i, &date)| {
                    let parallel_ratio = fill_ratio_parallel_universe(date, today);
                    let user_ratio = fill_ratio_user_universe(i, task.count_per_day, task.count_accum);
                    rsx! {
                        div {
                            class: "grid grid-cols-3 gap-1 items-center p-2 border border-gray-200 rounded-lg mb-0.5", // Bounding box styles
                            // Parallel Universe box
                            div {
                                class: "flex flex-col items-center",
                                div {
                                    class: "w-16 h-4 bg-gray-200 rounded overflow-hidden",
                                    div {
                                        class: "h-full bg-blue-400 transition-all duration-300",
                                        style: "width: {parallel_ratio * 100.0}%;"
                                    }
                                }
                            }

                            // Timeline date
                            div {
                                class: "flex flex-col items-center rounded-lg",
                                p {
                                    {
                                        format!("{} [{}]", date.format("%Y-%m-%d"), date.weekday())
                                    }
                                }

                                if let Some(daily_tasks) = &task.daily_tasks {
                                    if show_details() {
                                        p {
                                            class: "text-center text-sm text-gray-600",
                                            "{daily_tasks[i]}",
                                        }
                                    }
                                }
                            }

                            // Your Universe box
                            div {
                                class: "flex flex-col items-center",
                                div {
                                    class: "w-16 h-4 bg-gray-200 rounded overflow-hidden",
                                    div {
                                        class: "h-full bg-purple-400 transition-all duration-300",
                                        style: "width: {user_ratio * 100.0}%;"
                                    }
                                }
                            }
                        }
                    }
                })
            }

            div { class: "text-center font-semibold text-red-600", "🏁 Finish line!" }

            div {
                class: "text-center grid grid-cols-2 gap-4 justify-center",


                div {
                    class: "bg-blue-50 border border-blue-200 rounded-xl p-4 shadow",
                    p { "Accomplished: {parallel_accomplished:.1} {task.unit}" }
                    p { "Remaining: {parallel_remaining.max(0.0):.1} {task.unit}" }
                }

                div {
                    class: "bg-purple-50 border border-purple-200 rounded-xl p-4 shadow",
                    p { "Accomplished: {user_accomplished:.1} {task.unit}" }
                    p { "Remaining: {user_remaining.max(0.0):.1} {task.unit}" }
                }
            }
        }

        div {
            class: "p-6 max-w-5xl mx-auto space-y-6",

            div {
                class: "grid grid-cols-2 max-w-sm mx-auto",

                div {
                    class: "justify-self-end mr-2",
                    button {
                        class: "bg-gray-100 text-gray-800 font-medium py-2 px-4 rounded-lg transition-all hover:ring hover:ring-gray-300 hover:ring-offset-2",
                        onclick: move |_| {
                            navigator.push(Route::TaskList);
                        },
                        "⬅️ All Tasks",
                    }
                },

                div {
                    class: "justify-self-start ml-2",
                    button {
                        class: "bg-red-100 hover:bg-red-400 text-white font-medium py-2 px-4 rounded-lg transition-all hover:ring hover:ring-red-300 hover:ring-offset-2",
                        onclick: move |_| {
                            if n_clicks_on_remove() == 0 {
                                n_clicks_on_remove.set(1);
                            } else if n_clicks_on_remove() == 1 {
                                let mut tasks_signal = app_state.tasks.write();
                                if let Some(tasks_vec) = tasks_signal.as_mut() {
                                    tasks_vec.retain(|t| t.id != id);
                                }
                                navigator.push(Route::TaskList);
                            }
                        },
                        {
                            if n_clicks_on_remove() == 0 {"🗑 Remove Task"}
                            else if n_clicks_on_remove() == 1 {"Confirm Removing"}
                            else {"Error!"}
                        }
                    }
                }
            }
        }
    }
}
