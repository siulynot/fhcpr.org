use yew::prelude::*;
use web_sys::window;
use wasm_bindgen::{JsCast, closure::Closure};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let show_menu = use_state(|| false);
    let is_scrolled = use_state(|| false);

    // Detect scroll to apply .scrolled class
    {
        let is_scrolled = is_scrolled.clone();
        use_effect_with_deps(move |_| {
            let win = window().expect("window not available");
            let is_scrolled_c = is_scrolled.clone();

            let on_scroll = Closure::wrap(Box::new(move |_: web_sys::Event| {
                if let Some(w) = window() {
                    let y = w.scroll_y().unwrap_or(0.0);
                    is_scrolled_c.set(y > 60.0);
                }
            }) as Box<dyn FnMut(web_sys::Event)>);

            win.add_event_listener_with_callback(
                "scroll",
                on_scroll.as_ref().unchecked_ref(),
            ).ok();

            move || { drop(on_scroll); }
        }, ());
    }

    let toggle_menu = {
        let show_menu = show_menu.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            show_menu.set(!*show_menu);
        })
    };

    let scroll_to_section = {
        let show_menu = show_menu.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(target) = e.target() {
                if let Some(anchor) = target.dyn_ref::<web_sys::Element>() {
                    if let Some(href) = anchor.get_attribute("href") {
                        if href.starts_with('#') {
                            let id = href.strip_prefix('#').unwrap_or_default();
                            if let Some(win) = window() {
                                if let Some(doc) = win.document() {
                                    if id.is_empty() {
                                        win.scroll_to_with_x_and_y(0.0, 0.0);
                                    } else if let Ok(Some(el)) = doc.query_selector(&format!("#{}", id)) {
                                        el.scroll_into_view();
                                    }
                                    show_menu.set(false);
                                }
                            }
                        }
                    }
                }
            }
        })
    };

    html! {
        <nav class={classes!("navbar", "navbar-expand-lg", if *is_scrolled { "scrolled" } else { "" })}>
            <div class="container">
                <a class="navbar-brand" href="#" onclick={scroll_to_section.clone()}>
                    <img class="navbar-logo" src="images/fwdfirsthomecareincbrand/Seal_White.png" alt="First Home Care Center" />
                    <div class="brand-text-block">
                        <span class="brand-text">{"First Home Care Center"}</span>
                        <span class="brand-subtext">{"Est. 1984 · Servicios en el Hogar"}</span>
                    </div>
                </a>

                <button
                    class="navbar-toggler"
                    type="button"
                    aria-controls="navbarNav"
                    aria-expanded={if *show_menu { "true" } else { "false" }}
                    aria-label="Toggle navigation"
                    onclick={toggle_menu}
                >
                    <span class="navbar-toggler-icon"></span>
                </button>

                <div class={classes!("collapse", "navbar-collapse", if *show_menu { "show" } else { "" })} id="navbarNav">
                    <ul class="navbar-nav ms-auto">
                        <li class="nav-item">
                            <a class="nav-link" href="#" onclick={scroll_to_section.clone()}>{"Inicio"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#servicios" onclick={scroll_to_section.clone()}>{"Servicios"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#requisitos" onclick={scroll_to_section.clone()}>{"Requisitos"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#sobre-nosotros" onclick={scroll_to_section.clone()}>{"Sobre Nosotros"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="#contacto" onclick={scroll_to_section.clone()}>{"Contacto"}</a>
                        </li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
