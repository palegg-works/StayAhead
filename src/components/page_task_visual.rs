use crate::Route;
use crate::{AppState, NoSaveAppState, SyncMode};
use chrono::{Datelike, Duration, Local, NaiveDate, Timelike};
use dioxus::prelude::*;
use std::cmp::Ordering;
use super::css_preset::*;

const COLLAPSE_TO_TODAY_ICON: Asset = asset!("/assets/png/collapse_to_today.png");
const COLLAPSE_TO_DONE_ICON: Asset = asset!("/assets/png/collapse_to_done.png");

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
    let full_boxes = (count_accum / count_per_day) as usize;
    let remaining = count_accum % count_per_day;

    match index.cmp(&full_boxes) {
        Ordering::Less => 1.0,
        Ordering::Equal => remaining / count_per_day,
        Ordering::Greater => 0.0,
    }
}

#[component]
pub fn TaskVisual(id: i64) -> Element {
    let no_save_app_state = use_context::<NoSaveAppState>();
    let mut sync_msg = no_save_app_state.sync_msg;
    let mut sync_mode = no_save_app_state.sync_mode;
    let mut fire_push_after_deletion = no_save_app_state.fire_push_after_deletion;
    let mut fire_push_after_vis_change = use_signal(|| false);

    let mut app_state = use_context::<AppState>();
    let navigator = use_navigator();

    let task = (app_state.tasks)().and_then(|tasks| tasks.get(&id).cloned());

    if task.is_none() {
        return rsx! { 
            div {
                class: CSS_CONTENT_CARD,
                p { "Task not found." }
            }
        };
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
    let mut n_clicks_on_archive = use_signal(|| 0_i64);
    let mut collapse_to_today = use_signal(|| false);
    let mut collapse_to_done = use_signal(|| false);

    use_effect(move || {
        if fire_push_after_vis_change() {
            spawn({
                sync_mode.set(SyncMode::Pushing);
                async move {
                    let mut app_state = use_context::<AppState>();
                    match app_state.push().await {
                        Ok(_) => {
                            sync_msg.set(
                                "✅ Automatic push was successful after changing the visibility of a task!"
                                    .to_string(),
                            );
                            sync_mode.set(SyncMode::InSync);
                        }
                        Err(e) => {
                            sync_msg.set(format!(
                                "⚠️ Automatic push failed after changing the visibility of a task: {}",
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
            class: CSS_CONTENT_CARD,

            h2 { class: "text-xl font-bold text-center", "You told me that you want to ..."}
            h2 { class: "text-xl font-bold text-center", "\"{task.action} {task.count_per_day} {task.unit} every day\"" }
            h2 { class: "text-xl font-bold text-center", "Let's see how well you have done! 😎"}

            div { class: "text-center font-semibold text-green-700", "🚀 Let's Go!" }

            // A single grid container for the headers
            div {
                class: "grid grid-cols-3 gap-1 mb-4",
                div {
                    class: "flex flex-col items-center justify-top",
                    p { class: "font-medium text-blue-800", "🌌 Parallel Universe" }
                }

                div {
                    class: "flex flex-col justify-start",

                    if has_details() {
                        div {
                            class: format!(
                                       "w-full flex justify-center cursor-pointer rounded-lg text-center font-semibold transition-colors duration-200 {}",
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
                        div {
                            class: "w-full flex justify-center",
                            p { class: "font-medium text-gray-500", "📅 Timeline" }
                        }
                    }

                    div {
                        class: "w-full flex justify-center items-center mt-2",

                        p {
                            "Skip to "
                        }

                        button {
                            class: "p-0 w-9 transition-all duration-150 m-2",
                            onclick: move |_| {
                                collapse_to_today.set(!collapse_to_today());
                                collapse_to_done.set(false);
                            },
                            img {
                                src: COLLAPSE_TO_TODAY_ICON,
                                class: format!("cursor-pointer rounded-lg select-none {}",
                                    if collapse_to_today() {"bg-gray-300 shadow-inner"}
                                    else {"bg-gray-100 shadow-md hover:bg-gray-300"} ),
                            }
                        }

                        button {
                            class: "p-0 w-9 transition-all duration-150 m-2",
                            onclick: move |_| {
                                collapse_to_done.set(!collapse_to_done());
                                collapse_to_today.set(false);
                            },
                            img {
                                src: COLLAPSE_TO_DONE_ICON,
                                class: format!("cursor-pointer rounded-lg select-none {}",
                                    if collapse_to_done() {"bg-gray-300 shadow-inner"}
                                    else {"bg-gray-100 shadow-md hover:bg-gray-300"} ),
                            }
                        }
                    }
                }

                div {
                    class: "flex flex-col items-center justify-top",
                    p { class: "font-medium text-purple-800", "🪐 Your Universe" }
                }
            }

            // A single grid for each available date
            {
                let mut show_ellipsis = true;
                let mut n_done_days = task.count_accum / task.count_per_day;

                dates.iter().enumerate().map(move |(i, &date)| {
                    n_done_days -= 1.0;
                    let collapse_this = (collapse_to_today() && date < today) || (collapse_to_done() && n_done_days > 0.0);

                    if collapse_this {
                        if show_ellipsis {
                            show_ellipsis = false;

                            return rsx! {
                                div {
                                    class: "grid grid-cols-1 gap-1 items-center p-2 border border-gray-200 rounded-lg mb-0.5",
                                    div {
                                        class: "flex flex-col items-center",
                                        p { "..." }
                                    }
                                }
                            };
                        } else {
                            return rsx! {};
                        }
                    }

                    let parallel_ratio = fill_ratio_parallel_universe(date, today);
                    let user_ratio = fill_ratio_user_universe(i, task.count_per_day, task.count_accum);
                    rsx! {
                        div {
                            class: "grid grid-cols-3 gap-1 items-center p-2 border border-gray-200 rounded-lg mb-0.5",

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
                    p { "Accomplished: {parallel_accomplished:.1}" }
                    p { "Remaining: {parallel_remaining.max(0.0):.1}" }
                }

                div {
                    class: "bg-purple-50 border border-purple-200 rounded-xl p-4 shadow",
                    p { "Accomplished: {user_accomplished:.1}" }
                    p { "Remaining: {user_remaining.max(0.0):.1}" }
                }
            }
            
            div {
                class: {format!("p-6 max-w-5xl grid mx-auto {}", if task.archive {"grid-cols-3"} else {"grid-cols-2"})},
    
                div {
                    class: "flex flex-col justify-center mx-4",
                    button {
                        class: "bg-gray-100 text-gray-800 font-medium py-2 px-4 rounded-lg transition-all hover:ring hover:ring-gray-300 hover:ring-offset-2 cursor-pointer",
                        onclick: move |_| {
                            navigator.push(Route::TaskList);
                        },
                        "⬅️ All Tasks",
                    }
                },
    
                div {
                    class: "flex flex-col justify-center mx-4",
                    button {
                        class: "bg-red-100 hover:bg-red-400 text-white font-medium py-2 px-4 rounded-lg transition-all hover:ring hover:ring-red-300 hover:ring-offset-2 cursor-pointer",
                        onclick: move |_| {
                            if n_clicks_on_archive() == 0 {
                                n_clicks_on_archive.set(1);
                            } else if n_clicks_on_archive() == 1 {
                                let mut tasks_signal = app_state.tasks.write();
    
                                if let Some(tasks) = tasks_signal.as_mut() {
                                    for task in tasks.values_mut() {
                                        if task.id == id {
                                            task.archive = !task.archive;
                                        }
                                    }
                                }
    
                                fire_push_after_vis_change.set(true);
                                n_clicks_on_archive.set(0);
                            }
                        },
                        if task.archive {
                            if n_clicks_on_archive() == 0 {"📦 Activate Task"}
                            else if n_clicks_on_archive() == 1 {"Confirm Activation"}
                            else {"Error!"}
                        } else {
                            if n_clicks_on_archive() == 0 {"📦 Archive Task"}
                            else if n_clicks_on_archive() == 1 {"Confirm Archiving"}
                            else {"Error!"}
                        }
                    }
                }
    
                if task.archive {
                    div {
                        class: "flex flex-col justify-center mx-4",
                        button {
                            class: "bg-red-100 hover:bg-red-400 text-white font-medium py-2 px-4 rounded-lg transition-all hover:ring hover:ring-red-300 hover:ring-offset-2 cursor-pointer",
                            onclick: move |_| {
                                if n_clicks_on_remove() == 0 {
                                    n_clicks_on_remove.set(1);
                                } else if n_clicks_on_remove() == 1 {
                                    let mut tasks_signal = app_state.tasks.write();
                                    if let Some(tasks) = tasks_signal.as_mut() {
                                        tasks.remove(&id);
                                    }
    
                                    fire_push_after_deletion.set(true);
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
}
