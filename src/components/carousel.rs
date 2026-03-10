use yew::prelude::*;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;
use gloo::timers::callback::Timeout;

#[derive(Properties, Clone, PartialEq)]
struct SlideProps {
    image: String,
    heading: String,
    content: String,
    cta_label: String,
    cta_href: String,
    active: bool,
}

#[function_component(CarouselSlide)]
fn carousel_slide(props: &SlideProps) -> Html {
    html! {
        <div class={classes!("carousel-item", if props.active { "active" } else { "" })}>
            <img src={props.image.clone()} class="d-block w-100" alt={props.heading.clone()} />
            <div class="carousel-caption d-none d-md-block fade-in-on-load">
                <h2>{&props.heading}</h2>
                <p>{&props.content}</p>
                <a href={props.cta_href.clone()} class="hero-cta">
                    <i class="bi bi-arrow-right-circle-fill"></i>
                    {&props.cta_label}
                </a>
            </div>
        </div>
    }
}

#[function_component(Carousel)]
pub fn carousel() -> Html {
    let slides = vec![
        (
            "images/image4.png",
            "Servicios de Salud en el Hogar",
            "Una agencia sin fines de lucro, certificada por el Departamento de Salud de Puerto Rico y el Seguro Social Federal, dedicada a ofrecer atención médica de calidad en la comodidad de tu hogar.",
            "Ver Nuestros Servicios",
            "#servicios",
        ),
        (
            "images/image5.png",
            "Cuidado Personalizado para Ti",
            r#"Si eres beneficiario de "Medicare" o "Medicare Advantage" y cumples con los requisitos establecidos, podemos brindarte los servicios de atención médica que necesitas sin salir de casa."#,
            "Conocer Requisitos",
            "#requisitos",
        ),
        (
            "images/image6.png",
            "Atención Médica de Calidad",
            "En First Home Care Center creemos que todos merecen acceso a atención médica de calidad en la comodidad de su hogar. Déjanos ser tu socio en el camino hacia una vida más saludable.",
            "Contáctanos Hoy",
            "#contacto",
        ),
    ];

    let active_index = use_state(|| 0);
    let slides_count = slides.len();
    let pause_auto_rotation = use_state(|| false);

    // Auto-rotate
    {
        let active_index = active_index.clone();
        let pause_rotation = pause_auto_rotation.clone();

        use_effect_with_deps(move |_| {
            let interval = Interval::new(7000, move || {
                if !*pause_rotation {
                    let next = (*active_index + 1) % slides_count;
                    active_index.set(next);
                }
            });
            move || { drop(interval); }
        }, ());
    }

    let go_to_prev = {
        let active_index = active_index.clone();
        let pause_rotation = pause_auto_rotation.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            pause_rotation.set(true);
            let prev = if *active_index == 0 { slides_count - 1 } else { *active_index - 1 };
            active_index.set(prev);
            let pr = pause_rotation.clone();
            let t = Timeout::new(10000, move || { pr.set(false); });
            drop(t);
        })
    };

    let go_to_next = {
        let active_index = active_index.clone();
        let pause_rotation = pause_auto_rotation.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            pause_rotation.set(true);
            let next = (*active_index + 1) % slides_count;
            active_index.set(next);
            let pr = pause_rotation.clone();
            let t = Timeout::new(10000, move || { pr.set(false); });
            drop(t);
        })
    };

    let go_to_slide = {
        let active_index = active_index.clone();
        let pause_rotation = pause_auto_rotation.clone();

        Callback::from(move |e: MouseEvent| {
            pause_rotation.set(true);
            if let Some(target) = e.target() {
                if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                    if let Some(idx_str) = el.get_attribute("data-slide-to") {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            active_index.set(idx);
                        }
                    }
                }
            }
            let pr = pause_rotation.clone();
            let t = Timeout::new(10000, move || { pr.set(false); });
            drop(t);
        })
    };

    html! {
        <div id="carouselExampleIndicators" class="carousel slide">
            <div class="carousel-indicators">
                {slides.iter().enumerate().map(|(i, _)| html! {
                    <button
                        type="button"
                        data-bs-target="#carouselExampleIndicators"
                        data-slide-to={i.to_string()}
                        class={if i == *active_index { "active" } else { "" }}
                        aria-current={if i == *active_index { "true" } else { "false" }}
                        aria-label={format!("Slide {}", i + 1)}
                        onclick={go_to_slide.clone()}
                    ></button>
                }).collect::<Html>()}
            </div>

            <div class="carousel-inner">
                {slides.iter().enumerate().map(|(i, (img, h, c, label, href))| html! {
                    <CarouselSlide
                        image={img.to_string()}
                        heading={h.to_string()}
                        content={c.to_string()}
                        cta_label={label.to_string()}
                        cta_href={href.to_string()}
                        active={i == *active_index}
                    />
                }).collect::<Html>()}
            </div>

            <button class="carousel-control-prev" type="button" onclick={go_to_prev}>
                <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                <span class="visually-hidden">{"Anterior"}</span>
            </button>

            <button class="carousel-control-next" type="button" onclick={go_to_next}>
                <span class="carousel-control-next-icon" aria-hidden="true"></span>
                <span class="visually-hidden">{"Siguiente"}</span>
            </button>
        </div>
    }
}
