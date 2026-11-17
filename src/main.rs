use leptos::prelude::*;
use leptos_meta::provide_meta_context;

mod components;
use components::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        provide_meta_context();
        let year = js_sys::Date::new_0().get_full_year();
        view! {
            <main class="flex flex-col font-semibold bg-base-100">
                <div class="flex flex-col gap-12 p-6 mx-auto w-full max-w-3xl">
                    <div class="neo card card-xl">
                        <div class="p-3 card-body">
                            <h1 class="text-3xl card-title">"Fatih Demircan"</h1>
                            "Mechanical Engineer / Economics Undergraduate / Passion for Tinkering"
                        </div>
                    </div>
                    <Portfolio />
                    <div class="self-center text-xs">
                        {format!("© 2025-{year} Fatih Demircan")}
                    </div>
                </div>
                <Imprint />
            </main>
        }
    })
}
