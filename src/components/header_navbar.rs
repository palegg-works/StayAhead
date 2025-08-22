use crate::{AppState, NoSaveAppState, Route, SyncMode};
use dioxus::prelude::*;

const ICON_ALL_TASKS: Asset = asset!("/assets/png/all_tasks.png");
const ICON_NEW_TASKS: Asset = asset!("/assets/png/new_task.png");
const ICON_LOG_ACTION: Asset = asset!("/assets/png/log_action.png");
const ICON_QA: Asset = asset!("/assets/png/about.png");
const ICON_SETTING: Asset = asset!("/assets/png/sync.png");

#[component]
pub fn HeaderNavbar() -> Element {
    let current_route = use_route::<Route>();

    let no_save_app_state = use_context::<NoSaveAppState>();
    let mut sync_msg = no_save_app_state.sync_msg;
    let mut sync_mode = no_save_app_state.sync_mode;
    let mut fire_push_after_deletion = no_save_app_state.fire_push_after_deletion;

    let app_state = use_context::<AppState>();

    let current_route = use_route::<Route>();

    let tab_css = "flex justify-center items-center gap-2 px-2 py-3 text-blue-700 font-medium";
    let icon_css = "w-7 h-7";

    let active_class_str =
        "bg-white border border-gray-300 rounded-md shadow-sm text-blue-900 hover:text-blue-900";
    let inactive_class_str = "opacity-50 hover:opacity-100 hover:text-blue-900";

    use_effect(move || {
        if fire_push_after_deletion() {
            spawn({
                sync_mode.set(SyncMode::Pushing);
                async move {
                    let mut app_state = use_context::<AppState>();
                    match app_state.push().await {
                        Ok(_) => {
                            sync_msg.set(
                                "✅ Automatic push was successful after deleting a task!"
                                    .to_string(),
                            );
                            sync_mode.set(SyncMode::InSync);
                        }
                        Err(e) => {
                            sync_msg.set(format!(
                                "⚠️ Automatic push failed after deleting a task: {}",
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
            class: "min-h-screen relative",
            
            div {
                class: "fixed inset-0 bg-white",
            }

            header {
                class: "w-full flex justify-center fixed top-0 left-0 bg-white border-b-2 border-gray-300 z-10",
    
                nav {
                    class: "flex space-x-3 text-lg",
        
                    div {
                        class: {
                            let is_active = match &current_route {
                                Route::Director { pagename } if pagename == "TaskList" => true,
                                _ => false,
                            };
                            if is_active { active_class_str } else { inactive_class_str }
                        },
                        Link {
                            class: tab_css,
                            to: Route::Director { pagename: "TaskList".to_string() },
                            img {
                                src: ICON_ALL_TASKS,
                                class: icon_css,
                                alt: "All Tasks"
                            },
                            span {
                                class: "hidden sm:inline",
                                " Tasks"
                            }
                        }
                    }
        
                    div {
                        class: {
                            let is_active = match &current_route {
                                Route::Director { pagename } if pagename == "TaskCreate" => true,
                                _ => false,
                            };
                            if is_active { active_class_str } else { inactive_class_str }
                        },
                        Link {
                            class: tab_css,
                            to: Route::Director { pagename: "TaskCreate".to_string() },
                            img {
                                src: ICON_NEW_TASKS,
                                class: icon_css,
                                alt: "New Task"
                            },
                            span {
                                class: "hidden sm:inline",
                                " New"
                            }
                        }
                    }
        
                    div {
                        class: {
                            let is_active = match &current_route {
                                Route::Director { pagename } if pagename == "ActionLog" => true,
                                _ => false,
                            };
                            if is_active { active_class_str } else { inactive_class_str }
                        },
                        Link {
                            class: tab_css,
                            to: Route::Director { pagename: "ActionLog".to_string() },
                            img {
                                src: ICON_LOG_ACTION,
                                class: icon_css,
                                alt: "Log Action"
                            },
                            span {
                                class: "hidden sm:inline",
                                " Log"
                            }
                        }
                    }
        
                    div {
                        class: {
                            let is_active = match &current_route {
                                Route::Director { pagename } if pagename == "About" => true,
                                _ => false,
                            };
                            if is_active { active_class_str } else { inactive_class_str }
                        },
                        Link {
                            class: tab_css,
                            to: Route::Director { pagename: "About".to_string() },
                            img {
                                src: ICON_QA,
                                class: icon_css,
                                alt: "Q&A"
                            },
                            span {
                                class: "hidden sm:inline",
                                " Q&A"
                            }
                        }
                    }
        
                    div {
                        class: {
                            let is_active = match &current_route {
                                Route::Director { pagename } if pagename == "Setting" => true,
                                _ => false,
                            };
                            if is_active { active_class_str } else { inactive_class_str }
                        },
                        Link {
                            class: tab_css,
                            to: Route::Director { pagename: "Setting".to_string() },
                            img {
                                src: ICON_SETTING,
                                class: {
                                    match sync_mode() {
                                        SyncMode::NotSynced => format!("{} opacity-20 rounded-full", icon_css),
                                        SyncMode::Pushing => format!("{} bg-orange-200 rounded-full animate-spin", icon_css),
                                        SyncMode::Pulling => format!("{} bg-orange-200 rounded-full animate-spin", icon_css),
                                        SyncMode::InSync => format!("{} bg-green-400 rounded-full", icon_css),
                                        SyncMode::Failed => format!("{} bg-red-400 rounded-full", icon_css),
                                    }
                                },
                                alt: "Sync"
                            },
                            span {
                                class: "hidden sm:inline",
                                " Sync"
                            }
                        }
                    }
        
                }
            }

            main {
                class: "mt-18",
                Outlet::<Route> {}
            }
        }
        
        }
}
