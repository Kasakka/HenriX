use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        div {
            div(class="flex"){
                p { "Hello, World!" }
                p { "Hello, World!" }
                p { "Hello, World!" }
                p { "Hello, World!" }
            }
        }
    });
}