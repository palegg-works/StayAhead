use super::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn HeaderNavbar() -> Element {
    let current_route = use_route::<Route>();
    let is_active = |target: &Route| current_route == *target;

    let tab_css = "flex justify-center px-2 py-5 text-blue-700 font-medium";

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
                    "📋 All Tasks"
                }
            }

            div {
                class: if is_active(&Route::TaskCreate) { active_class_str } else { inactive_class_str },
                Link {
                    class: tab_css,
                    to: Route::TaskCreate,
                    "🆕 New Task"
                }
            }

            div {
                class: if is_active(&Route::ActionLog) { active_class_str } else { inactive_class_str },
                Link {
                    class: tab_css,
                    to: Route::ActionLog,
                    "📝 Log Action"
                }
            }

            div {
                class: if is_active(&Route::About) { active_class_str } else { inactive_class_str },
                Link {
                    class: tab_css,
                    to: Route::About,
                    "🧐 Q&A"
                }
            }

        }

        main {
            class: "mt-6",
            Outlet::<Route> {}
        }
    }
}
