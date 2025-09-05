use leptos::prelude::*;
use leptos_meta::provide_meta_context;

mod app;
use app::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        provide_meta_context();
        view! {
            <main class="flex justify-center font-semibold">
                <div class="max-w-2xl flex flex-col items-center">
                    <div class="inline-flex mt-6 px-6">
                        <div>
                            <h1 class="text-4xl">"Fatih Demircan"</h1>
                            <div>
                                "Mechanical engineer - with a degree in economics and
                                passion for programming."
                            </div>
                        </div>
                    </div>
                    <div class="w-full flex flex-col divide-y-1 divide-base-content/10">
                        <About />
                        <Portfolio />
                        <Skills />
                        <Contact />
                    </div>
                    <div class="my-3 badge badge-neutral p-0 px-3">
                        "Made with" <img src="public/fig/leptos.svg" class="h-5 aspect-auto" alt="leptos_logo" />
                    </div>
                    <div class="my-3 mb-12 text-xs">"© 2025 - Fatih Demircan"</div>
                </div>
            </main>
        }
    })
}
