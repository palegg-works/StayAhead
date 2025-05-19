use super::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn HeaderNavbar() -> Element {
    let current_route = use_route::<Route>();
    let is_active = |target: &Route| current_route == *target;

    let active_class_str =
        "bg-white border border-gray-300 rounded-md shadow-sm text-blue-900 px-2 py-4 text-blue-700 hover:text-blue-900 font-medium";
    let inactive_class_str =
        "opacity-50 hover:opacity-100 px-2 py-4 text-blue-700 hover:text-blue-900 font-medium";
    //let link_class_str = "text-blue-700 hover:text-blue-900 font-medium";

    rsx! {
        nav {
            class: "bg-gray-100 shadow p-1 flex justify-center space-x-2 py-4",

            div {
                Link {
                    class: if is_active(&Route::TaskList) { active_class_str } else { inactive_class_str },
                    to: Route::TaskList,
                    "📋 All Tasks"
                }
            }

            div {
                Link {
                    class: if is_active(&Route::TaskCreate) { active_class_str } else { inactive_class_str },
                    to: Route::TaskCreate,
                    "🆕 New Task"
                }
            }

            div {
                Link {
                    class: if is_active(&Route::ActionLog) { active_class_str } else { inactive_class_str },
                    to: Route::ActionLog,
                    "📝 Log Action"
                }
            }

            div {
                Link {
                    class: if is_active(&Route::About) { active_class_str } else { inactive_class_str },
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
