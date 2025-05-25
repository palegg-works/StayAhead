use super::routes::Route;
use crate::states::{AppState, SerializableState};
use dioxus::prelude::*;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::states::import_data;

#[component]
pub fn TaskList() -> Element {
    let app_state = use_context::<AppState>();
    let app_state_for_export = use_context::<AppState>();

    // These variables are created for accepting imports for web targets
    let mut app_state_import = use_context::<AppState>();
    let mut imported_serializable_state = use_signal::<Option<SerializableState>>(|| None);

    use_memo(move || {
        if let Some(state) = imported_serializable_state() {
            if let Ok(state) = AppState::try_from(state) {
                app_state_import.tasks.set((state.tasks)());
            }
        }
    });

    let has_tasks = use_memo(move || {
        if let Some(tasks) = (app_state_import.tasks)() {
            !tasks.is_empty()
        } else {
            false
        }
    });

    #[cfg(any(target_os = "android", target_os = "ios"))]
    let div_export_import_data_buttons = rsx! {};

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let div_export_import_data_buttons = rsx! {
        div {
            class: "flex justify-center gap-4",

            button {
                class: "font-semibold py-2 px-2 rounded bg-blue-300 hover:bg-blue-700 text-white cursor-pointer transition-colors duration-300",
                onclick: move |_| {
                    app_state_for_export.export_data();
                },
                "Export",
            },

            button {
                class: "font-semibold py-2 px-2 rounded bg-blue-300 hover:bg-blue-700 text-white cursor-pointer transition-colors duration-300",
                onclick: move |_| {

                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(imported_serializable_state) = import_data() {
                        if let Ok(imported_app_state) = TryInto::<AppState>::try_into(imported_serializable_state) {
                            app_state_import.tasks.set((imported_app_state.tasks)());
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    import_data(move |parsed: SerializableState| {
                        imported_serializable_state.set(Some(parsed));
                    });
                },
                "Restore",
            },
        }
    };

    rsx! {
        div {
            class: "p-6 max-w-2xl mx-auto space-y-6",

            if has_tasks() {
                div {
                    p {
                        class: "font-semibold text-gray-800 text-lg",
                        {format!("🗂 You have {} task(s):", (app_state.tasks)().unwrap().len())}
                    }

                    div {
                        class: "grid grid-cols-1 gap-4",
                        {
                            (app_state.tasks)().unwrap().iter().map(|task| {
                                let id = task.id;
                                rsx! {
                                    Link {
                                        to: Route::TaskVisual { id },
                                        class: "relative block bg-white shadow-md rounded-xl p-4 border border-gray-100 hover:shadow-lg transition-shadow hover:ring-2 hover:ring-blue-300",
                                        h3 {
                                            class: "text-lg font-semibold text-blue-800",
                                            {
                                                if task.name.is_some() {
                                                    task.name.clone().unwrap()
                                                } else {
                                                    format!("{} {} {}", task.action, task.count_per_day, task.unit)
                                                }
                                            }
                                        }

                                        p {
                                            class: "text-sm text-gray-600",
                                            { format!("Done so far: {:.1} {}", task.count_accum, task.unit) }
                                        }

                                        p {
                                            class: "text-sm text-gray-500",
                                            { format!("From {} to (deadline) {}", task.start, task.end) }
                                        }

                                        p {
                                            class: "text-sm text-gray-500",
                                            {
                                                let summary_msg = task.effective_dow
                                                    .iter()
                                                    .map(|d| d.to_string())
                                                    .collect::<Vec<_>>()
                                                    .join(", ");
                                                format!("Effective days: {}", summary_msg)
                                            }
                                        }
                                    }
                                }
                            })

                        }
                    }
                }
            } else {
                p {
                    class: "text-gray-600 text-center",
                    "You do not have any tasks yet. Challenge yourself by adding a new task!",
                }
            }

            { div_export_import_data_buttons }
        }
    }
}
