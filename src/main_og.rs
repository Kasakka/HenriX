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
            div(class="intro"){
                h1(class="intro--title") {"HENRI MATERO"}
                p(class="intro--lead") {"Tähän joku pikku insert meikästä ja ehkä pinkkejä keywordejä?"}
            }
            section(class="cards"){
                div(class="flex cards--container"){
                    a(class="card card-1") {
                        img(class="card--image", src="assets/images/roses.jpg")
                        h3(class="card--title"){"Jotain"}
                    }
                    a(class="card card-2") {
                        img(class="card--image", src="assets/images/koodari.jpg")
                        h3(class="card--title"){"CV"}
                    }
                    a(class="card card-3") {
                        img(class="card--image", src="assets/images/yeller.jpg")
                        h3(class="card--title"){"Bio"}
                    }
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