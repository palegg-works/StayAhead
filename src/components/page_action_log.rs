use crate::states::{AppState, MOTIVATIONAL_MSGS};
use chrono::Local;
use dioxus::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn get_motivational_msg() -> String {
    let timestamp = Local::now().timestamp();
    let mut hasher = DefaultHasher::new();
    timestamp.hash(&mut hasher);
    let hash = hasher.finish();
    let index = (hash as usize) % MOTIVATIONAL_MSGS.len();
    MOTIVATIONAL_MSGS[index].to_string()
}

#[component]
pub fn ActionLog() -> Element {
    let mut app_state = use_context::<AppState>();

    let mut submit_return_msg = use_signal(|| "".to_string());

    let has_tasks = use_signal(|| {
        if let Some(tasks) = (app_state.tasks)() {
            !tasks.is_empty()
        } else {
            false
        }
    });

    let mut selected_task_index = use_signal(|| if has_tasks() { Some(0_usize) } else { None });

    let mut count_done = use_signal(|| {
        if selected_task_index().is_some() {
            1.0_f32
        } else {
            0.0_f32
        }
    });

    let enable_submit = use_memo(move || selected_task_index().is_some() && count_done() > 0.0_f32);

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
                                    if let Some(index) = tasks.iter().position(|task| task.id == id) {
                                        selected_task_index.set(Some(index));
                                    }
                                }
                            }
                        },

                        if let Some(tasks) = (app_state.tasks)() {
                            {
                                tasks.iter().filter(|task| !task.archive).map(|task| rsx! {
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
                            let unit_str = if let Some(i) = selected_task_index() {
                                " ".to_string() + &(app_state.tasks)().unwrap()[i].unit
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
                            if let Some(i) = selected_task_index() {
                                let mut tasks_write = app_state.tasks.write();
                                if let Some(tasks_mut) = tasks_write.as_mut() {
                                    if let Some(task) = tasks_mut.get_mut(i) {
                                        task.count_accum += count_done();

                                        // If debugging
                                        //let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                                        //submit_return_msg.set(format!("✅ Logged! ({})", now));

                                        // Use motivational messages in production
                                        submit_return_msg.set(get_motivational_msg());
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
