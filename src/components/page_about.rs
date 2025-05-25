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
                        "For each task, Stay Ahead shows two timelines: ",
                        strong { "Parallel Universe" },
                        " (planned progression) and ",
                        strong { "Your Universe" },
                        " (your actual effort). This helps you visualize the gap and stay motivated. ",
                        span {
                            class: "inline-block bg-yellow-200 text-yellow-800 px-2 py-0.5 rounded-md font-bold italic", // Highlighted with a background
                            "Make this universe (timeline) your best one!"
                        },
                    }
                }

                div {
                    h2 {  class: "font-semibold", "Can I see your source code?" }
                    p {
                        "Absolutely. Please visit ",
                        a {
                            href: "https://github.com/palegg-works/StayAhead",
                            target: "_blank",
                            class: "text-blue-600 hover:text-blue-800 underline font-semibold",
                            "Github (StayAhead from Palegg Works)"
                        },
                        "."
                    }
                }

                div {
                    h2 {  class: "font-semibold", "Cool. Can I use this on a different platform?" }
                    p { "Yes. Currently there is a desktop release for MacOS. Please refer the release page on Github. I have personally built and installed the Android version for myself. Open a ticket if this is needed."}
                }

                div {
                    h2 { class: "font-semibold", "Does the app collect any user data?" }
                    p { "No. All your data is stored locally on your device. There is no cloud syncing or server-side tracking involved, at least for the time being. For example, if you are using the web version, you will loose your data if you clear the cache. The desktop app will offer better data persistency as it is saved on disk." }
                }

                div {
                    h2 { class: "font-semibold", "Who made this?" }
                    p {
                        "I am an independent developer and I decided to go by ",
                        span {
                            class: "italic text-indigo-700 font-semibold px-1 py-0.5 rounded-sm hover:bg-indigo-100 transition-colors duration-200",
                            "Palegg Works"
                        },
                        " to promote mindful productivity and consistency."
                    }
                }

                div {
                    h2 { class: "font-semibold", "What is the current version?" }
                    p { "Version: {VERSION}" }
                }
            }
        }
    }
}
