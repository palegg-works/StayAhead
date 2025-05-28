use crate::{AppState, Route, SyncMode};
use dioxus::prelude::*;

const ICON_ALL_TASKS: Asset = asset!("/assets/png/all_tasks.png");
const ICON_NEW_TASKS: Asset = asset!("/assets/png/new_task.png");
const ICON_LOG_ACTION: Asset = asset!("/assets/png/log_action.png");
const ICON_QA: Asset = asset!("/assets/png/about.png");
const ICON_SETTING: Asset = asset!("/assets/png/sync.png");

#[component]
pub fn HeaderNavbar() -> Element {
    let app_state = use_context::<AppState>();
    let current_route = use_route::<Route>();
    let is_active = |target: &Route| current_route == *target;

    let tab_css = "flex justify-center items-center gap-2 px-2 py-5 text-blue-700 font-medium";
    let icon_css = "w-7 h-7";

    let active_class_str =
        "bg-white border border-gray-300 rounded-md shadow-sm text-blue-900 hover:text-blue-900";
    let inactive_class_str = "opacity-50 hover:opacity-100 hover:text-blue-900";

    rsx! {
        nav {
            class: "bg-gray-100 shadow flex justify-center space-x-2 py-1",

            div {
                class: if is_active(&Route::TaskList) { active_class_str } else { inactive_class_str },
                Link {
                    class: tab_css,
                    to: Route::TaskList,
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
                class: if is_active(&Route::TaskCreate) { active_class_str } else { inactive_class_str },
                Link {
                    class: tab_css,
                    to: Route::TaskCreate,
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
                class: if is_active(&Route::ActionLog) { active_class_str } else { inactive_class_str },
                Link {
                    class: tab_css,
                    to: Route::ActionLog,
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
                class: if is_active(&Route::About) { active_class_str } else { inactive_class_str },
                Link {
                    class: tab_css,
                    to: Route::About,
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
                class: if is_active(&Route::Setting) { active_class_str } else { inactive_class_str },
                Link {
                    class: tab_css,
                    to: Route::Setting,
                    img {
                        src: ICON_SETTING,
                        class: {
                            match (app_state.sync_mode)() {
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

        main {
            class: "mt-6",
            Outlet::<Route> {}
        }
    }
}
