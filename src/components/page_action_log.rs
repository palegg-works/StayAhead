use crate::states::MOTIVATIONAL_MSGS;
use crate::{AppState, NoSaveAppState, SyncMode};
use chrono::Local;
use dioxus::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn random_motivational_msg() -> String {
    let timestamp = Local::now().timestamp();
    let mut hasher = DefaultHasher::new();
    timestamp.hash(&mut hasher);
    let hash = hasher.finish();
    let index = (hash as usize) % MOTIVATIONAL_MSGS.len();
    MOTIVATIONAL_MSGS[index].to_string()
}

#[component]
pub fn ActionLog() -> Element {
    let no_save_app_state = use_context::<NoSaveAppState>();
    let mut sync_msg = no_save_app_state.sync_msg;
    let mut sync_mode = no_save_app_state.sync_mode;
    let mut fire_push = use_signal(|| false);

    let mut app_state = use_context::<AppState>();

    let mut submit_return_msg = use_signal(|| "".to_string());

    let has_tasks = use_signal(|| {
        if let Some(tasks) = (app_state.tasks)() {
            !tasks.is_empty()
        } else {
            false
        }
    });

    let mut selected_task_id = use_signal(|| {
        if has_tasks() {
            Some(
                (app_state.tasks)()
                    .unwrap()
                    .values()
                    .next()
                    .expect("Error")
                    .id,
            )
        } else {
            None
        }
    });

    let mut count_done = use_signal(|| {
        if selected_task_id().is_some() {
            1.0_f32
        } else {
            0.0_f32
        }
    });

    let enable_submit = use_memo(move || {
        selected_task_id().is_some()
            && count_done() > 0.0_f32
            && sync_mode() != SyncMode::Pushing
            && sync_mode() != SyncMode::Pulling
    });

    use_effect(move || {
        if fire_push() {
            spawn({
                sync_mode.set(SyncMode::Pushing);
                async move {
                    let mut app_state = use_context::<AppState>();
                    match app_state.push().await {
                        Ok(_) => {
                            sync_msg
                                .set("✅ Automatic push was successful after logging!".to_string());
                            sync_mode.set(SyncMode::InSync);
                        }
                        Err(e) => {
                            sync_msg.set(format!(
                                "⚠️ Automatic push failed after creating logging: {}",
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
        if has_tasks() {
            div {
                class: "p-6 max-w-xl mx-auto space-y-6 bg-white rounded-xl shadow",

                h2 {
                    class: "text-lg font-bold",
                    "💪 Good job! Let's log your accomplishment!"
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        "Select a task:"
                    }

                    select {
                        class: "block w-full border border-gray-300 rounded-md shadow-sm p-2",
                        onchange: move |e| {
                            if let Ok(id) = e.data().value().parse::<i64>() {
                                if let Some(tasks) = (app_state.tasks)() {
                                    selected_task_id.set(Some(id));
                                }
                            }
                        },

                        if let Some(tasks) = (app_state.tasks)() {
                            {
                                tasks.values().filter(|task| !task.archive).map(|task| rsx! {
                                    option {
                                        value: "{task.id}",
                                        {
                                            if let Some(task_name) = &task.name {
                                                task_name.clone()
                                            } else {
                                                format!("{} {} {}", task.action, task.count_per_day, task.unit)
                                            }
                                        }
                                    }
                                })
                            }
                        } else {
                            option { disabled: true, "No tasks available" }
                        }
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium text-gray-700 mb-1",
                        {
                            let unit_str = if let Some(id) = selected_task_id() {
                                if let Some(t) = (app_state.tasks)().unwrap().get(&id) {
                                    " ".to_string() + &t.unit
                                } else {
                                    " *unit error*".to_string()
                                }
                            } else {
                                " ".to_string()
                            };

                            format!("How many{} did you accomplish? (Try fractions as well)", unit_str)
                        }
                    }

                    input {
                        r#type: "number",
                        min: "0",
                        step: "any",
                        class: "block w-full border border-gray-300 rounded-md shadow-sm p-2",
                        value: "{count_done}",
                        oninput: move |e| {
                            if let Ok(val) = e.value().parse::<f32>() {
                                count_done.set(val);
                            } else {
                                count_done.set(0.0_f32);
                            }
                        }
                    }
                }

                div {
                    class: "pt-4 flex items-center gap-x-4",
                    button {
                        disabled: "{!enable_submit()}",
                        class: format_args!(
                            "font-semibold py-2 px-4 rounded transition-colors duration-300 {}",
                            if enable_submit() {
                                "bg-blue-600 hover:bg-blue-700 text-white cursor-pointer"
                            } else {
                                "bg-gray-300 text-gray-500 cursor-not-allowed"
                            }
                        ),
                        onclick: move |_| {
                            if let Some(id) = selected_task_id() {
                                let mut tasks_write = app_state.tasks.write();
                                if let Some(tasks_mut) = tasks_write.as_mut() {
                                    if let Some(task) = tasks_mut.get_mut(&id) {
                                        task.count_accum += count_done();

                                        // Use motivational messages in production
                                        submit_return_msg.set(random_motivational_msg());

                                        fire_push.set(true);
                                    }
                                }
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
            }
        } else {
            p {
                class: "text-gray-600 text-center",
                "You do not have any tasks yet. Challenge yourself by adding a new task in a different tab!",
            }
        }
    }
}
