use super::*;
use leptos::{ev::MouseEvent, html::Div, tachys::dom::event_target};
use web_sys::HtmlDivElement;

#[component]
fn Placeholder() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col gap-2">
            <div class="skeleton h-32 w-full"></div>
            <div class="skeleton h-4 w-1/2"></div>
            <div class="skeleton h-4 w-full"></div>
            <div class="skeleton h-4 w-full"></div>
            <div class="skeleton h-4 w-full"></div>
        </div>
    }
}

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
    view! {
        <Card title="Portfolio".to_string()>
            <Carousel length=5>
                <Slide title="AHRS Module".to_string()>
                    <article class="text-justify">
                        <img
                            class="rounded-box w-full sm:w-[40%] float-none sm:float-left mb-2 sm:mr-2"
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
                {move || {
                    (0..4)
                        .into_iter()
                        .map(|_| {
                            view! {
                                <Slide title=format!("Soon to be updated")>
                                    <Placeholder />
                                </Slide>
                            }
                        })
                        .collect_view()
                }}
            </Carousel>
        </Card>
    }
}
