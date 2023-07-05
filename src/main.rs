use sycamore::prelude::*;
use sycamore_router::{Route, Router, RouterProps, HistoryIntegration};
use web_sys::{HtmlInputElement, KeyboardEvent, console};

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/projects")]
    Projects,
    #[to("/blog/<id>")]
    Blog(String),
    #[not_found]
    NotFound,
}

fn main() {
    sycamore::render(App);
}

#[component]
fn App<'a, G: Html>(cx: Scope) -> View<G> {
    // Initialize application state from localStorage.
    let local_storage = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .expect("user has not enabled localStorage");

    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    Nav {}
                    main {
                        (match route.get().as_ref() {
                            AppRoutes::Index => view! { cx,
                                "This is the index page"
                            },
                            AppRoutes::Projects => view! { cx,
                                "About this website"
                            },
                            AppRoutes::Blog(id) => {
                                view! { cx,
                                    BlogBody(content=id) {}
                                }
                            }
                            AppRoutes::NotFound => view! { cx,
                                "404 Not Found"
                            },
                        })
                    }
                    Footer {}
                }
            }
        )
    }
}

#[component]
fn Nav<'a, G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // ...
    }
}

#[component]
fn Footer<'a, G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // ...
    }
}

#[derive(Props)]
struct BlogProps<'a, G: Html> {
    // _content: &'a str,
    children: Children<'a, G>,
    content: &'a String,
}

#[component]
fn BlogBody<'a, G: Html>(cx: Scope, props: BlogProps<'a, G>) -> View<G> {
    let blog_content = props.content.clone();
    view! { cx,
        p { (blog_content) }
    }
}

