use super::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Card title="Contact".to_string()>
            <article class="text-center">
                // <img
                // class="mt-3 h-5 aspect-auto justify-self-center"
                // src=mail
                // alt="mail"
                // />
                <p class="pt-6 text-xs sm:text-sm">
                    "*the address is embedded as an image in order to avoid misuse
                    through webscraping!"
                </p>
            </article>
        </Card>
    }
}
