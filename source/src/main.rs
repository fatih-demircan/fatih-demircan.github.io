use leptos::prelude::*;
use leptos_meta::provide_meta_context;

#[component]
fn Card(title: String, children: Children) -> impl IntoView {
    view! {
        <div class="box-border w-full flex-col justify-center items-center">
            <div class="card sm:card-lg w-full">
                <div class="card-body">
                    <div class="card-title">{title}</div>
                    {children()}
                </div>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(|| {
        provide_meta_context();
        view! {
            <main class="flex justify-center">
                <div class="max-w-3xl flex flex-col items-center sm:text-lg">
                    <div class="inline-flex mt-6">
                        // <Favicon />
                        <h1 class="text-4xl">"Fatih Demircan"</h1>
                    </div>
                    <div class="w-full flex flex-col divide-y-1 divide-base-content/10">
                        <Card title="Short Bio".to_string()>
                            <div class="text-justify indent-6 sm:indent-12">
                                "I'm a mechanical engineer - who also holds a degree in economics - with
                                a passion for programming."
                            </div>
                            <div class="text-justify indent-6 sm:indent-12">
                                "After high school, I initially chose to study industrial engineering but
                                opted for economics at the University of Cologne. However, driven by my
                                passion for technology, I decided to pursue engineering as a career. I
                                completed my Bachelor's in mechanical engineering at the Technical
                                University of Braunschweig, followed by a Master's degree. Throughout my
                                studies, I focused on methodological depth, eventually specializing in
                                mechatronics. This allowed me to combine traditional engineering with my
                                passion for programming, giving me a broader perspective and enabling me
                                to move beyond siloed thinking."
                            </div>
                            <div class="text-justify indent-6 sm:indent-12">
                                "In case you have interesting projects or potential business inquiries,
                                feel free to contact me."
                            </div>
                        </Card>
                        <Card title="Contact".to_string()>
                            <article class="text-center">
                                // <img
                                // class="mt-3 h-5 aspect-auto justify-self-center"
                                // src=mail
                                // alt="mail"
                                // />
                                <p class="pt-6 text-xs sm:text-sm">
                                    *the address is embedded as an image in order to avoid misuse
                                    through webscraping!
                                </p>
                            </article>
                        </Card>
                    </div>
                    <div class="py-6 text-xs sm:text-sm">"© 2025 - Fatih Demircan"</div>
                </div>
            </main>
        }
    })
}

// <div class="bg-red-300">"Test"</div>
// <button class="btn btn-primary btn-soft">Test</button>
