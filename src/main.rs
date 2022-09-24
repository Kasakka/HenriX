use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        main {
            img(class="card--image", src="assets/images/closed.jpg", alt="under construction")
        }
    });
}