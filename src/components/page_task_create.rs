use crate::states::AppState;
use crate::states::MyTask;
use chrono::{Local, NaiveDate, Weekday};
use dioxus::prelude::*;

static ALL_WEEKDAYS: [Weekday; 7] = [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

#[component]
pub fn TaskCreate() -> Element {
    let mut app_state = use_context::<AppState>();

    let today_str = Local::now().date_naive().to_string();
    let tomorrow_str = (Local::now().date_naive() + chrono::Duration::days(1)).to_string();
    let one_week_later_str = (Local::now().date_naive() + chrono::Duration::days(7)).to_string();

    let mut action = use_signal(|| "".to_string());
    let mut count_per_day = use_signal(|| 1.0_f32);
    let mut unit = use_signal(|| "".to_string());
    let mut start = use_signal(|| today_str.to_string());
    let mut end = use_signal(|| one_week_later_str.to_string());
    let mut submit_return_msg = use_signal(|| "".to_string());
    let mut selected_dow = use_signal(|| ALL_WEEKDAYS.to_vec());

    let enable_submit = use_memo(move || {
        !action().is_empty()
            && count_per_day() > 0.0_f32
            && !unit().is_empty()
            && !selected_dow.is_empty()
    });

    rsx! {
        div {
            class: "p-6 max-w-xl mx-auto space-y-4 bg-white rounded-xl shadow",

            div {
                class: "space-y-2 text-gray-800",
                p { class: "text-lg font-bold", "😎 Let's add a task to challenge yourself!" }
                p { "Below are some examples:" }
                ul { class: "list-disc list-inside space-y-1 text-sm",
                    li { "I want to (read) (10) (minutes) per day" }
                    li { "I want to (write) (5) (paragraphs) per day" }
                }
            }

            div {
                class: "text-sm text-gray-800 font-medium",
                "Every day, I want to"
            }

            div {
                class: "flex gap-x-4 items-center",

                div {
                    input {
                        r#type: "text",
                        class: "w-full border border-gray-300 rounded-md shadow-sm p-2",
                        placeholder: "Action",
                        value: "{action}",
                        oninput: move |e| action.set(e.value())
                    }
                }

                div {
                    class: "w-1/6",
                    input {
                        r#type: "number",
                        step: "any",
                        min: "0",
                        class: "w-full border border-gray-300 rounded-md shadow-sm p-2",
                        placeholder: "Count",
                        value: "{count_per_day}",
                        oninput: move |e| {
                            if let Ok(num) = e.value().parse::<f32>() {
                                count_per_day.set(num)
                            } else {
                                count_per_day.set(0.0_f32)
                            }
                        }
                    }
                }

                div {
                    class: "w-1/3",
                    input {
                        r#type: "text",
                        class: "w-full border border-gray-300 rounded-md shadow-sm p-2",
                        placeholder: "Unit",
                        autocomplete: "off",
                        spellcheck: "false",
                        value: "{unit}",
                        oninput: move |e| unit.set(e.value().clone())
                    }
                }
            }

            div {
                label { class: "block text-sm font-medium text-gray-700", "Start Date" }
                input {
                    r#type: "date",
                    class: "mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2",
                    value: "{start}",
                    oninput: move |e| start.set(e.value().clone())
                }
            }

            div {
                label { class: "block text-sm font-medium text-gray-700", "End Date" }
                input {
                    r#type: "date",
                    min: "{tomorrow_str}",
                    class: "mt-1 block w-full border border-gray-300 rounded-md shadow-sm p-2",
                    value: "{end}",
                    oninput: move |e| end.set(e.value().clone())
                }
            }

            div {
                label {
                    class: "block text-sm font-medium text-gray-700",
                    "This task will be effective on these days:",
                },

                div {
                    class: "flex flex-wrap gap-4 mt-2",
                    {
                        ALL_WEEKDAYS.iter().map(|d| {
                            let short_label = d.to_string();
                            let selected = selected_dow.read().contains(d);

                            rsx! {
                                label {
                                    class: "inline-flex items-center space-x-2 cursor-pointer select-none",
                                    input {
                                        r#type: "checkbox",
                                        checked: selected,
                                        onchange: move |evt| {
                                            let is_now_checked = evt.value() == "true";
                                            if is_now_checked {
                                                if !selected_dow.read().contains(d) {
                                                    selected_dow.write().push(d.to_owned());
                                                }
                                            } else if selected_dow.read().contains(d) {
                                                selected_dow.write().retain(|e| e != d);
                                            }
                                        }
                                    },
                                    span { "{short_label}" }
                                }
                            }
                        })
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
                        if let (Ok(start_date), Ok(end_date)) = (
                            NaiveDate::parse_from_str(&start(), "%Y-%m-%d"),
                            NaiveDate::parse_from_str(&end(), "%Y-%m-%d")
                        ) {
                            selected_dow.write().sort_by_key(|d| d.num_days_from_monday());

                            let task = MyTask {
                                id: chrono::Utc::now().timestamp_millis(),
                                action: action().clone(),
                                count_per_day: count_per_day(),
                                unit: unit().clone(),
                                count_accum: 0.0_f32,
                                start: start_date,
                                end: end_date,
                                effective_dow: selected_dow(),
                            };

                            let should_insert = app_state.tasks.read().is_some();

                            if should_insert {
                                app_state.tasks.write().as_mut().unwrap().push(task);
                            } else {
                                app_state.tasks.set(Some(vec![task]));
                            }

                            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                            submit_return_msg.set(format!("✅ Created a task ({})", now));
                        } else {
                            submit_return_msg.set("❌ Task creation failed! Please check dates!".to_string());
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
    }
}
