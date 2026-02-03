use super::*;

#[component]
fn PortfolioElement(children: Children, title: String) -> impl IntoView {
    view! {
        <div class="card-sm grid grid-cols-1 w-full">
            <div class="card-body">
                <div class="card-title">{title}</div>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn Portfolio() -> impl IntoView {
    let modal_one: NodeRef<leptos::html::Dialog> = NodeRef::new();
    view! {
        <ImgCarousel modal_one=modal_one />
        <Card title="Some previous work".to_string()>
            <PortfolioElement title="Android App - RAgent".to_string()>
                <article class="text-justify">
                    <button
                        on:click=move |_| {
                            let _ = modal_one.get().unwrap().show_modal();
                        }
                        class="w-full sm:w-[30%] float-right grid grid-cols-2 gap-1 mb-2 sm:ml-2"
                    >
                        <img src="public/fig/ragent-app/01.webp" alt="ragent-app_01" />
                        <img src="public/fig/ragent-app/02.webp" alt="ragent-app_02" />
                    </button>
                    <p>
                        "The job of a ramp agent is to coordinate the complete turnaround of an aircraft. A major part of this role involves tracking the start and end times of each operation, such as deboarding, refueling, boarding, and others. In addition, the ramp agent must keep track of cumulative passenger numbers per section, as well as the loading of baggage and cargo in the respective holds, since these figures affect the aircraft’s center of gravity and are therefore essential for the cockpit crew to set the trim. This app is intended as a dummy design to demonstrate what a suitable digital counterpart to the currently paper-intensive workflow might look like. "
                        <a class="link" href="https://github.com/fatih-demircan/assets">
                            "[Link to the .apk-files]"
                        </a>"."
                    </p>
                </article>
            </PortfolioElement>
            <PortfolioElement title="AHRS-Module".to_string()>
                <article class="text-justify">
                    <img
                        class="w-full sm:w-[40%] float-none sm:float-left mb-2 sm:mr-2 rounded-xl"
                        src="public/fig/ahrs_board.webp"
                        alt="ahrs_board"
                    />
                    <p>
                        "I have developed an AHRS (Attitude and Heading Reference System) module that integrates a GNSS receiver and a pressure sensor. My work encompasses both the PCB design and the accompanying embedded software. The PCB was created using KiCad, while the firmware is built with the Rust Embassy framework. The system is intended for use in future projects - primarily in an unmanned aerial vehicle - where precise attitude and positional data are essential."
                    </p>
                </article>
            </PortfolioElement>
            <PortfolioElement title="Cellular Automaton".to_string()>
                <article class="text-justify">
                    <img
                        class="w-full sm:w-[60%] float-none sm:float-right mb-2 sm:ml-2"
                        src="public/fig/fhp_gui.png"
                        alt="fhp_gui"
                    />
                    <p>
                        "This small application was coded initially as part of a student project. It is a Lattice gas automaton with a hexagonal grid (model introduced by Uriel Frisch, Brosl Hasslacher and Yves Pomeau in 1986), simulating a flow around a NACA-Profile in a tunnel. Further enhancements are going to be done soon!"
                    </p>
                </article>
            </PortfolioElement>
            <PortfolioElement title="RegRS - Rust-based Python package".to_string()>
                <article class="text-justify">
                    <img
                        class="w-full sm:w-[60%] float-none sm:float-left mb-2 sm:mr-2"
                        src="public/fig/regrs_summary.png"
                        alt="regrs_summary"
                    />
                    <p>
                        "Computing the predicted R² by iteratively leaving one row from, exog. and endog. data and computing the error based on the left out data row. And since each iteration step is independent it is very much suitable to be parallilized. Hence, I wrote a corresponding function in Rust and compiled in to a .whl-File so that it can be easily used in Python."
                    </p>
                </article>
            </PortfolioElement>
            <PortfolioElement title="2D Heatflux Sim".to_string()>
                <article class="text-justify">
                    <img
                        class="w-full sm:w-[60%] float-none sm:float-right mb-2 sm:ml-2"
                        src="public/fig/heatflux_gui.png"
                        alt="heatflux_gui"
                    />
                    <p>
                        "This 2D heat conduction simulation was created in the course 'Object
                        -oriented simulation methods in Thermodynamics and Fluid Dynamics' (Institute of Thermodynamics, TU Braunschweig)."
                    </p>
                </article>
            </PortfolioElement>
        </Card>
    }
}

#[component]
fn ImgCarousel(modal_one: NodeRef<leptos::html::Dialog>) -> impl IntoView {
    view! {
        <dialog node_ref=modal_one class="modal">
            <div class="modal-box bg-transparent px-0 mx-0 shadow-none">
                <button
                    class="badge rounded-full p-0 leading-none aspect-square absolute right-0 top-0"
                    on:click=move |_| {
                        let _ = modal_one.get().unwrap().close();
                    }
                >
                    <svg
                        class="fill-current"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 -960 960 960"
                    >
                        <path d="m336-280 144-144 144 144 56-56-144-144 144-144-56-56-144 144-144-144-56 56 144 144-144 144 56 56ZM480-80q-83 0-156-31.5T197-197q-54-54-85.5-127T80-480q0-83 31.5-156T197-763q54-54 127-85.5T480-880q83 0 156 31.5T763-763q54 54 85.5 127T880-480q0 83-31.5 156T763-197q-54 54-127 85.5T480-80Zm0-80q134 0 227-93t93-227q0-134-93-227t-227-93q-134 0-227 93t-93 227q0 134 93 227t227 93Zm0-320Z" />
                    </svg>
                </button>
                <div class="carousel carousel-center max-w-2xl space-x-4">
                    <For
                        each=move || (1..=7).into_iter()
                        key=|idx| idx.clone()
                        children=move |idx| {
                            view! {
                                <div class="carousel-item">
                                    <img
                                        class="w-60"
                                        src=format!("public/fig/ragent-app/0{idx}.webp")
                                        alt=format!("ragent-app_0{idx}")
                                    />
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </dialog>
    }
}
