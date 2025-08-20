use leptos::prelude::*;
use leptos_meta::provide_meta_context;

fn main() {
    leptos::mount::mount_to_body(|| {
        provide_meta_context();
        view! {
            <div class="bg-red-300">"Test"</div>
            <button class="btn btn-primary btn-soft">Test</button>
        }
    })
}
