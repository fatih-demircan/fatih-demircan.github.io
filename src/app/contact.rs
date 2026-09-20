use super::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Card title="Contact".to_string()>
            <div class="pt-6 text-justify indent-6 text-base-content/70"></div>
        </Card>
    }
}
