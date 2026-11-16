use super::*;

#[component]
pub fn Imprint() -> impl IntoView {
    view! {
        <button
            class="w-full rounded-none border-0 transition-shadow btn neo active:neo-in"
            onclick="imprint.showModal()"
        >
            "Imprint"
        </button>
        <dialog id="imprint" class="modal modal-bottom">
            <div class="place-items-center text-center modal-box">
                <h3 class="text-lg font-bold">"Imprint"</h3>
                <p class="py-9 text-sm font-normal">
                    "Information pursuant to § 5 DDG"<br /> <br /> "Fatih Demircan"<br />
                    "Musterstraße 1"<br /> "12345 Musterstadt"<br /> "Germany"<br /> <br />
                    "Contact"<br /> "Email: mail@example.com"
                </p>
            </div>
            <form method="dialog" class="modal-backdrop">
                <button>"close"</button>
            </form>
        </dialog>
    }
}
