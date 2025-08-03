use daisy_rsx::*;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)] // <---- add the #[layout] attribute
    #[route("/")]
    Home,
    #[route("/tools")]
    Tools,
    #[route("/projects")]
    Projects,
    #[route("/devlog")]
    DevLog,
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "",
            div {
                class: "bg-base-100 py-40 text-center",
                h1 {
                    "Graham Lancaster"
                }
                h4 {
                    "Engineer | Rustacean | Climber"
                }
            }
            div {
            class: "flex flex-wrap md:flex-nowrap bg-base-100 text-center",
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
                Link { to: "/",
                    a { class: "btn btn-ghost text-l", "GL" }
                }
            }
            div { class: "navbar-center lg:flex",
                ul { class: "menu menu-horizontal px-1",
                    li {
                        Link { to: "/tools",
                            a { "Tools" }
                        }
                    }
                    li {
                        Link { to: "/projects",
                            a { "Projects" }
                        }
                    }
                    li {
                        Link { to: "/devlog",
                            a { "Dev Log" }
                        }
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
        Outlet::<Route> {}
    }
}
#[component]
fn Tools() -> Element {
    let tools = vec![
        "tool a", "tool b", "tool c", "tool d", "tool e", "tool f", "tool g",
    ];
    rsx! {
        div {
            class: "flex flex-row",
        div {
            class: "border bg-red-300 w-1/6 h-dvh p-2 flex flex-col gap-2",
            for tool in &tools {
                button {
                    class: "border bg-red-500 rounded-md",
                    "{tool}"
                }
            }

        }
        div {
            class: "flex flex-wrap grow border bg-red-300 p-5 gap-4",
            for tool in &tools {
                button {
                    class: "border bg-red-500 rounded-md w-1/4 grow",
                    "{tool}"
                }
            }
        }
        }
    }
}

#[component]
fn Projects() -> Element {
    let projects = vec![
        "project a",
        "project b",
        "project c",
        "project d",
        "project e",
        "project f",
        "project g",
    ];
    rsx! {
        div {
            class: "flex flex-wrap grow border bg-red-300 p-5 gap-4 h-dvh",
            for project in &projects {
                button {
                    class: "border bg-red-500 rounded-md w-1/4 grow",
                    "{project}"
                }
            }
        }

    }
}

#[component]
fn DevLog() -> Element {
    let logs = vec![
        "log a", "log b", "log c", "log d", "log e", "log f", "log g",
    ];
    rsx! {
        div {
            class: "flex flex-col grow border bg-red-300 p-5 gap-4 h-dvh",
            for log in &logs {
                button {
                    class: "border bg-red-500 rounded-md h-1/3 grow",
                    "{log}"
                }
            }
        }
    }
}
