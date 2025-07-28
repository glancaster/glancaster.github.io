use daisy_rsx::*;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        NavBar {}
        Home {}
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "",
        div {
            class: "bg-base-100 p-10 text-center",
            h1 {
                "Graham Lancaster"
            }
            h4 {
                "Engineer | Rustacean | Climber"
            }
        }
        div {
            class: "flex bg-base-100 text-center",
            div { class: "card bg-primary text-primary-content m-2",
                div { class: "card-body",
                    h2 { class: "card-title", "Latest Tool" }
                    p {
                        "Title of the tool \n then its short description of the tool"
                    }
                    div { class: "card-actions justify-end",
                        button { class: "btn", "View" }
                    }
                }
            }
            div { class: "card bg-primary text-primary-content m-2",
                div { class: "card-body",
                    h2 { class: "card-title", "Latest Project" }
                    p {
                        "A card component has a figure, a body part, and inside body there are title and actions parts"
                    }
                    div { class: "card-actions justify-end",
                        button { class: "btn", "View" }
                    }
                }
            }
            div { class: "card bg-primary text-primary-content m-2",
                div { class: "card-body",
                    h2 { class: "card-title", "Latest Post" }
                    p {
                        "A card component has a figure, a body part, and inside body there are title and actions parts"
                    }
                    div { class: "card-actions justify-end",
                        button { class: "btn", "View" }
                    }
                }
            }
        }
        }
    }
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "navbar bg-base-100 shadow-sm",
            div { class: "navbar-start",
                a { class: "btn btn-ghost text-l", "GL" }
            }
            div { class: "navbar-center lg:flex",
                ul { class: "menu menu-horizontal px-1",
                    li {
                        a { "Tools" }
                    }
                    li {
                        a { "Projects" }
                    }
                    li {
                        a { "Posts" }
                    }
                }
            }
            div { class: "navbar-end",
                ul { class: "menu menu-horizontal px-1",
                    li {
                        a { class: "btn btn-ghost",
                            "L" }
                    }
                    li {
                        a { class: "btn btn-ghost",
                            "G" }
                    }
                }
            }
        }
    }
}
