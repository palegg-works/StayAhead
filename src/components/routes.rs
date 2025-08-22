use super::header_navbar::HeaderNavbar;
use super::page_about::About;
use super::page_action_log::ActionLog;
use super::page_setting::Setting;
use super::page_task_create::TaskCreate;
use super::page_task_list::TaskList;
use super::page_task_visual::TaskVisual;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(HeaderNavbar)]
    #[route("/")]
    TaskList,

    #[route("/:pagename")]
    Director { pagename: String },
}

#[component]
pub fn Director(pagename: String) -> Element {
    let parts: Vec<&str> = pagename.split('/').collect();
    match parts[0] {
        "TaskVisual" => {
            let id = match parts.get(1).and_then(|s| s.parse::<i64>().ok()) {
                Some(valid_id) => valid_id,
                None => return rsx!(TaskList {}),
            };
            rsx!(TaskVisual { id: id })
        }
        "TaskCreate" => rsx!(TaskCreate {}),
        "ActionLog" => rsx!(ActionLog {}),
        "About" => rsx!(About {}),
        "Setting" => rsx!(Setting {}),
        "TaskList" => rsx!(TaskList {}),
        _ => rsx!(TaskList {}),
    }
}
