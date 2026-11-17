use super::*;

#[component]
pub fn Imprint() -> impl IntoView {
    let (open, set_open) = signal(false);
    let dialog = NodeRef::<leptos::html::Dialog>::new();

    // Split literals so no contiguous string appears in the wasm binary
    let name = || ["Fatih", " ", "Demircan"].concat();
    let street = || ["Sticksfeld", " ", "7"].concat();
    let city = || ["30539", " ", "Hannover"].concat();
    let email = || ["fatihdmrcn", "@", "outlook", ".com"].concat();

    view! {
        <button
            class="w-full rounded-none border-0 transition-shadow btn neo active:neo-in"
            on:click=move |_| {
                set_open.set(true);
                if let Some(d) = dialog.get() {
                    let _ = d.show_modal();
                }
            }
        >
            "Imprint"
        </button>
        <dialog node_ref=dialog class="modal modal-bottom" on:close=move |_| set_open.set(false)>
            <div class="place-items-center text-center modal-box">
                <h3 class="text-lg font-bold">"Imprint"</h3>
                <Show when=move || open.get()>
                    <p class="py-4 text-sm font-normal">
                        "Information in accordance with German law (§ 5 DDG)"<br /> <br /> {name()}
                        <br /> {street()}<br /> {city()}<br /> "Germany"<br /> <br /> "Contact"
                        <br /> "Email: " <a class="link" href=format!("mailto:{}", email())>
                            {email()}
                        </a>
                    </p>
                </Show>
            </div>
            <form method="dialog" class="modal-backdrop">
                <button>"close"</button>
            </form>
        </dialog>
    }
}
