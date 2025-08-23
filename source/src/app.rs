use super::*;

mod about;
mod contact;
mod portfolio;
mod skills;

pub use about::About;
pub use contact::Contact;
pub use portfolio::Portfolio;
pub use skills::Skills;

#[component]
fn Card(children: Children, title: String) -> impl IntoView {
    view! {
        <div class="box-border w-full flex-col justify-center items-center">
            <div class="card w-full">
                <div class="card-body">
                    <div class="card-title">{title}</div>
                    {children()}
                </div>
            </div>
        </div>
    }
}
