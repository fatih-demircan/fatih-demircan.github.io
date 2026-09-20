use leptos::prelude::*;
use leptos_meta::provide_meta_context;

mod app;
use app::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        provide_meta_context();
        view! {
            <main class="flex justify-center font-semibold">
                <div class="max-w-3xl flex flex-col items-center">
                    <div class="inline-flex my-6 px-6 w-full">
                        <div>
                            <h1 class="text-4xl">"Fatih Demircan"</h1>
                            <div>
                                "Mechanical Engineer / Passion for Tinkering / Economics Undergraduate"
                            </div>
                        </div>
                    </div>
                    <div class="w-full flex flex-col divide-y divide-base-content/10">
                        <img
                            class="w-full rounded-none md:rounded-2xl"
                            src="public/fig/fd.webp"
                            alt="me"
                        />
                        <Portfolio />
                        <Skills />
                    </div>
                    <div class="my-3 badge badge-neutral p-0 px-3">
                        "Made with"
                        <img
                            src="public/fig/leptos.svg"
                            class="h-5 aspect-auto"
                            alt="leptos_logo"
                        />
                    </div>
                    <div class="my-3 mb-12 text-xs">"© 2025 - Fatih Demircan"</div>
                </div>
            </main>
        }
    })
}
