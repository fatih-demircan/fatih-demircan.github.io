use super::*;
use leptos::{ev::MouseEvent, html::Div, tachys::dom::event_target};
use web_sys::HtmlDivElement;

#[component]
fn Slide(children: Children, title: String) -> impl IntoView {
    view! {
        <div class="carousel-item box-border w-full">
            <div class="card-sm grid grid-cols-1 w-full">
                <div class="card-body">
                    <div class="card-title">{title}</div>
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
fn Carousel(children: Children, length: i32) -> impl IntoView {
    let div_ref: NodeRef<Div> = NodeRef::new();
    let (pos, set_pos) = signal(0);

    let _scroll = move |new_pos: f64| {
        let div = div_ref.get().unwrap();
        let x = (new_pos * (div.scroll_width() - div.client_width()) as f64) / (length - 1) as f64;
        div.scroll_to_with_x_and_y(x, 0.);
    };

    let scroll_to = move |ev: MouseEvent| {
        let new_pos = event_target_value(&ev).parse::<f64>().unwrap();
        _scroll(new_pos);
    };

    let scroll_left = move |_| {
        if pos.get() == 0 {
            _scroll((length - 1) as f64)
        } else {
            _scroll((pos.get() - 1) as f64)
        }
    };
    let scroll_right = move |_| {
        if pos.get() == length - 1 {
            _scroll(0.)
        } else {
            _scroll((pos.get() + 1) as f64)
        }
    };

    let onscroll = move |event| {
        let div = event_target::<HtmlDivElement>(&event);
        let a = div.scroll_left() as f32; // actual position for scrollLeft
        let b = (div.scroll_width() - div.client_width()) as f32; // max. value for scrollLeft
        *set_pos.write() = ((a / b) * (length as f32 - 1.)).round() as i32;
    };

    view! {
        <div class="flex-1 w-full relative">
            <div
                node_ref=div_ref
                on:scroll=onscroll
                class="w-full carousel overflow-y-hidden items-start"
            >
                {children()}
            </div>
            <div class="absolute w-full bottom-0 translate-y-1/2 flex gap-1 sm:gap-3 justify-center">
                <For each=move || (0..length) key=|x| x.clone() let(x)>
                    <button
                        value=x
                        aria-label="slide"
                        on:click=scroll_to
                        class="btn btn-circle size-3"
                        class=("bg-base-content/70", move || x == pos.get())
                    ></button>
                </For>
            </div>
            <button
                aria-label="scrollLeft"
                class="absolute left-0 top-1/2 -translate-x-1/2 -translate-y-1/2 size-6"
                on:click=scroll_left
            >
                <svg
                    viewBox="0 -960 960 960"
                    xmlns="http://www.w3.org/2000/svg"
                    class="size-full fill-base-content/70"
                >
                    <path d="M560-280 360-480l200-200v400Z" />
                </svg>
            </button>
            <button
                aria-label="scrollRight"
                class="absolute right-0 top-1/2 translate-x-1/2 -translate-y-1/2 size-6"
                on:click=scroll_right
            >
                <svg
                    viewBox="0 -960 960 960"
                    xmlns="http://www.w3.org/2000/svg"
                    class="size-full fill-base-content/70"
                >
                    <path d="M400-280v-400l200 200-200 200Z" />
                </svg>
            </button>
        </div>
    }
}

#[component]
pub fn Portfolio() -> impl IntoView {
    let modal_one: NodeRef<leptos::html::Dialog> = NodeRef::new();
    view! {
        <CarouselOne modal_one=modal_one />
        <Card title="Portfolio".to_string()>
            <Carousel length=5>
                <Slide title="Android App - RAgent".to_string()>
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
                            "Details on this app will be provided soon. Since it is just a dummy app, no core functionalities were implemented. However, if there is interest in testing it out, the .apk-files can be downloaded "
                            <a class="link" href="https://github.com/fatih-demircan/assets">
                                here
                            </a>"."
                        </p>
                    </article>
                </Slide>
                <Slide title="AHRS Module".to_string()>
                    <article class="text-justify">
                        <img
                            class="w-full sm:w-[50%] float-none sm:float-left mb-2 sm:mr-2"
                            src="public/fig/ahrs_board.webp"
                            alt="ahrs_board"
                        />
                        <p>
                            "I've designed an AHRS module (Attitude Heading Reference
                             System), also integrating a GNSS chip as well as a pressure
                             sensor. My work covers the design of the PCB as well as the
                             corresponding code. The PCB was designed using KiCad whereas
                             the embedded code makes use the Rust crate Embassy. The goal
                             is to utilize this in future projects, primarily for a
                             drone."
                        </p>
                    </article>
                </Slide>
                <Slide title="Cellular Automaton".to_string()>
                    <article class="text-justify">
                        <img
                            class="w-full float-none sm:float-left mb-2 sm:mr-2"
                            src="public/fig/fhp_gui.png"
                            alt="fhp_gui"
                        />
                        <p>
                            "This small application was coded initially as part of a student project. It is a Lattice gas automaton with a hexagonal grid (model introduced by Uriel Frisch, Brosl Hasslacher and Yves Pomeau in 1986), simulating a flow around a NACA-Profile in a tunnel. Further enhancements are going to be done soon!"
                        </p>
                    </article>
                </Slide>
                <Slide title="RegRS - Rust-based Python package".to_string()>
                    <article class="text-justify">
                        <img
                            class="w-full float-none sm:float-left mb-2 sm:mr-2"
                            src="public/fig/regrs_summary.png"
                            alt="regrs_summary"
                        />
                        <p>
                            "Computing the predicted R² by iteratively leaving one row from, exog. and endog. data and computing the error based on the left out data row. And since each iteration step is independent it is very much suitable to be parallilized. Hence, I wrote a corresponding function in Rust and compiled in to a .whl-File so that it can be easily used in Python."
                        </p>
                    </article>
                </Slide>
                <Slide title="2D Heatflux Sim".to_string()>
                    <article class="text-justify">
                        <img
                            class="w-full sm:w-[60%] float-none sm:float-left mb-2 sm:mr-2"
                            src="public/fig/heatflux_gui.png"
                            alt="heatflux_gui"
                        />
                        <p>
                            "This 2D heat conduction simulation was created in the course 'Object
                            -oriented simulation methods in Thermodynamics and Fluid Dynamics' (Institute of Thermodynamics, TU Braunschweig)."
                        </p>
                    </article>
                </Slide>
            </Carousel>
        </Card>
    }
}

#[component]
fn CarouselOne(modal_one: NodeRef<leptos::html::Dialog>) -> impl IntoView {
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
