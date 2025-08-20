use leptos::prelude::*;
use leptos_meta::provide_meta_context;

fn main() {
    leptos::mount::mount_to_body(|| {
        provide_meta_context();
        view! { <div>"Test"</div> }
    })
}
