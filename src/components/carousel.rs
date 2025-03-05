use yew::prelude::*;
use gloo_timers::callback::Interval;
use wasm_bindgen::JsCast;

#[derive(Properties, Clone, PartialEq)]
struct SlideProps {
    image: String,
    heading: String,
    content: String,
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
            </div>
        </div>
    }
}

#[function_component(Carousel)]
pub fn carousel() -> Html {
    // Slides data
    let slides = vec![
        (
            "images/image4.png",
            "SERVICIOS DE SALUD EN EL HOGAR",
            "Una agencia sin fines de lucro, dedicada a ofrecer servicios de salud de calidad en la comodidad de tu hogar o lugar de residencia. Nos enorgullece estar certificados por el Departamento de Salud del Estado Libre Asociado de Puerto Rico y el Seguro Social Federal, lo que significa que puedes confiar en nosotros para proporcionar servicios de atención médica excepcionales.",
        ),
        (
            "images/image5.png",
            "CUIDADO PERSONALIZADO",
            r#"Nuestro objetivo es brindar servicios de salud personalizados y adaptados a las necesidades individuales de cada paciente. Si eres beneficiario de "Medicare" y "Medicare Advantage" y cumples con los requisitos establecidos, podemos ayudarte a obtener los servicios de atención médica que necesitas sin tener que salir de casa."#,
        ),
        (
            "images/image6.png",
            "ATENCIÓN MÉDICA DE CALIDAD",
            "En First Home Care Center, Inc., creemos que todos merecen acceso a atención médica de calidad en la comodidad de su hogar. Déjanos ser tu socio en tu camino hacia una vida más saludable y feliz. ¡Contáctanos hoy mismo para obtener más información sobre nuestros servicios!",
        ),
    ];
    
    let active_index = use_state(|| 0);
    let slides_count = slides.len();
    
    // Auto-rotate carousel
    {
        let active_index = active_index.clone();
        let slides_count = slides_count;
        
        use_effect_with_deps(move |_| {
            let interval = Interval::new(5000, move || {
                let next_index = (*active_index + 1) % slides_count;
                active_index.set(next_index);
            });
            
            // Cleanup
            move || { drop(interval); }
        }, ());
    }
    
    let go_to_prev = {
        let active_index = active_index.clone();
        let slides_count = slides_count;
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let prev_index = if *active_index == 0 { 
                slides_count - 1 
            } else { 
                *active_index - 1 
            };
            active_index.set(prev_index);
        })
    };
    
    let go_to_next = {
        let active_index = active_index.clone();
        let slides_count = slides_count;
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let next_index = (*active_index + 1) % slides_count;
            active_index.set(next_index);
        })
    };
    
    let go_to_slide = {
        let active_index = active_index.clone();
        
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target() {
                if let Some(element) = target.dyn_ref::<web_sys::Element>() {
                    if let Some(index_str) = element.get_attribute("data-slide-to") {
                        if let Ok(index) = index_str.parse::<usize>() {
                            active_index.set(index);
                        }
                    }
                }
            }
        })
    };
    
    html! {
        <div id="carouselExampleIndicators" class="carousel slide" data-bs-ride="carousel">
            <div class="carousel-indicators">
                {slides.iter().enumerate().map(|(index, _)| {
                    html! {
                        <button 
                            type="button" 
                            data-bs-target="#carouselExampleIndicators" 
                            data-slide-to={index.to_string()} 
                            class={if index == *active_index { "active" } else { "" }} 
                            aria-current={if index == *active_index { "true" } else { "false" }}
                            aria-label={format!("Slide {}", index + 1)}
                            onclick={go_to_slide.clone()}
                        ></button>
                    }
                }).collect::<Html>()}
            </div>
            
            <div class="carousel-inner">
                {slides.iter().enumerate().map(|(index, (image, heading, content))| {
                    html! {
                        <CarouselSlide 
                            image={image.to_string()} 
                            heading={heading.to_string()} 
                            content={content.to_string()} 
                            active={index == *active_index}
                        />
                    }
                }).collect::<Html>()}
            </div>
            
            <button class="carousel-control-prev" type="button" data-bs-target="#carouselExampleIndicators" onclick={go_to_prev}>
                <span class="carousel-control-prev-icon" aria-hidden="true"></span>
                <span class="visually-hidden">{"Previous"}</span>
            </button>
            
            <button class="carousel-control-next" type="button" data-bs-target="#carouselExampleIndicators" onclick={go_to_next}>
                <span class="carousel-control-next-icon" aria-hidden="true"></span>
                <span class="visually-hidden">{"Next"}</span>
            </button>
        </div>
    }
}
