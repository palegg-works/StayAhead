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
                    h2 {  class: "font-semibold", "Can I see your source code?" }
                    p { "Absolutely. The source code can be accessed from ", a { href: "https://github.com/palegg-works/StayAhead", target: "_blank", "Github" }, "." }
                }

                div {
                    h2 {  class: "font-semibold", "Cool. Can I use this on a different platform?" }
                    p { "Yes. Currently there is a desktop release for MacOS. Please refer the release page on Github. I have personally built and installed the Android version for myself. Please open a ticket if this is needed."}
                }

                div {
                    h2 { class: "font-semibold", "Does the app collect any user data?" }
                    p { "No. All your data is stored locally on your device. There is no cloud syncing or server-side tracking involved, at least for the time being. For example, if you are using the web version, you will loose your data if you clear the cache. The desktop app will offer better data persistency as it is saved on disk." }
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
