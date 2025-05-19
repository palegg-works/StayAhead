use dioxus::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "p-6 max-w-2xl mx-auto space-y-6 text-gray-800",

            h1 {
                class: "text-2xl font-bold text-blue-700",
                "📘 About Stay Ahead"
            }

            div {
                class: "space-y-4",

                div {
                    h2 { class: "font-semibold", "What is Stay Ahead?" }
                    p { "Stay Ahead is a minimalist habit-tracking and self-discipline app designed to help you build consistency, track progress, and visualize your goals. Its biggest difference is that it allows your insufficient or excessive progress to carry over through time. It is like you are racing another parallel universe!" }
                }

                div {
                    h2 { class: "font-semibold", "How does the timeline work?" }
                    p {
                        "For each task, Stay Ahead shows two timelines: "
                        strong { "Parallel Universe" }
                        " (planned progression) and "
                        strong { "Your Universe" }
                        " (your actual effort). This helps you visualize the gap and stay motivated. "
                        strong { "Make this universe (timeline) your best one!" }
                    }
                }

                div {
                    h2 { class: "font-semibold", "Is my data safe?" }
                    p { "Yes. All your data is stored locally on your device. There is no cloud syncing or server-side tracking involved, at least for the time being." }
                }

                div {
                    h2 { class: "font-semibold", "Who made this?" }
                    p { "Stay Ahead is an independent app developed by Palegg Works to promote mindful productivity and consistency." }
                }

                div {
                    h2 { class: "font-semibold", "What is the current version?" }
                    p { "Version: {VERSION}" }
                }
            }
        }
    }
}
