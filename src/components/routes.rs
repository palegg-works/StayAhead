use super::header_navbar::HeaderNavbar;
use super::page_about::About;
use super::page_action_log::ActionLog;
use super::page_task_create::TaskCreate;
use super::page_task_list::TaskList;
use super::page_task_visual::TaskVisual;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(HeaderNavbar)]
    #[route("/")]
    TaskList,

    #[route("/task_visual/:id")]
    TaskVisual { id: i64 },

    #[route("/task_create")]
    TaskCreate,

    #[route("/action_log")]
    ActionLog,

    #[route("/about")]
    About,
}
