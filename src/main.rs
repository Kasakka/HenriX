use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        div {
            div(class="flex"){
                p { a(href="#test"){"Hello, World!"} }
                p { "Hello, World!" }
                p { "Hello, World!" }
                p { "Hello, World!" }
            }
            div(id="test") {
                p {"Tadaa"}
            }
        }
    });
}