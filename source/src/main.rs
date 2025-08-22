use leptos::prelude::*;
use leptos_meta::provide_meta_context;

mod app;
use app::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        provide_meta_context();
        view! {
            <main class="flex justify-center">
                <div class="max-w-2xl flex flex-col items-center">
                    <div class="inline-flex mt-6">
                        // <Favicon />
                        <h1 class="text-4xl">"Fatih Demircan"</h1>
                    </div>
                    <div class="w-full flex flex-col divide-y-1 divide-base-content/10">
                        <About />
                        <Skills />
                        <Contact />
                    </div>
                    <div class="py-6 text-xs">"© 2025 - Fatih Demircan"</div>
                </div>
            </main>
        }
    })
}
