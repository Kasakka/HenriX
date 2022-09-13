use sycamore::prelude::*;

#[component]
fn NavBar<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        header(class="topnav") {
            a(href="#", class="topnav--logo") {"HenriX"}
            a(href="#") {"Link"}
            a(href="#") {"Link"}
            a(href="#footer") {"Footer"}
        }
    }
}

#[component]
fn Content<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        section(class="content"){
            div(class="flex"){

            }
        }
    }
}


#[component]
fn Footer<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        footer(id="footer", class="footer") {
            a(href="#") {"Footiboogi"}
        }
    }
}


fn main() {
    sycamore::render(|cx| view! { cx,
        main {
            NavBar { }
            Content { }
            Footer { }
        }
    });
}