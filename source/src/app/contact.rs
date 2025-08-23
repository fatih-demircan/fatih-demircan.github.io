use super::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Card title="Contact".to_string()>
            <div class="pt-6 text-justify indent-6 text-base-content/70">
                "In case you have interesting projects or potential business inquiries, feel free to contact me. Contact details will be provided very soon."
            </div>
        </Card>
    }
}
