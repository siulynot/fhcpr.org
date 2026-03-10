use yew::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::carousel::Carousel;
use crate::components::services::Services;
use crate::components::requirements::Requirements;
use crate::components::about::About;
use crate::components::footer::Footer;
use web_sys::window;
use gloo::timers::callback::Timeout;
use wasm_bindgen::{JsCast, closure::Closure};

#[function_component(Home)]
pub fn home() -> Html {
    let show_scroll_button = use_state(|| false);

    {
        let show_button = show_scroll_button.clone();

        use_effect_with_deps(move |_| {
            let win = window().expect("window not available");
            let doc = win.document().expect("document not available");

            // Scroll-to-top button visibility
            let show_btn = show_button.clone();
            let check_scroll = {
                let w = win.clone();
                Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let y = w.scroll_y().unwrap_or(0.0);
                    show_btn.set(y > 300.0);
                }) as Box<dyn FnMut(web_sys::Event)>)
            };

            doc.add_event_listener_with_callback(
                "scroll",
                check_scroll.as_ref().unchecked_ref(),
            ).expect("Failed to add scroll listener");

            // Scroll-reveal animations
            let handle_animations = {
                let w = win.clone();
                Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Some(document) = w.document() {
                        if let Ok(elements) = document.query_selector_all(".animate-on-scroll") {
                            let wh = w.inner_height().unwrap().as_f64().unwrap_or(0.0);
                            for i in 0..elements.length() {
                                if let Some(node) = elements.get(i) {
                                    if let Some(el) = node.dyn_ref::<web_sys::Element>() {
                                        let rect = el.get_bounding_client_rect();
                                        if rect.top() - wh <= 40.0 {
                                            let _ = el.class_list().add_1("fade-in-element");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(web_sys::Event)>)
            };

            doc.add_event_listener_with_callback(
                "scroll",
                handle_animations.as_ref().unchecked_ref(),
            ).expect("Failed to add animation listener");

            // Trigger on load after short delay
            let wc = win.clone();
            let timeout = Timeout::new(120, move || {
                if let Some(document) = wc.document() {
                    if let Ok(elements) = document.query_selector_all(".animate-on-scroll") {
                        let wh = wc.inner_height().unwrap().as_f64().unwrap_or(0.0);
                        for i in 0..elements.length() {
                            if let Some(node) = elements.get(i) {
                                if let Some(el) = node.dyn_ref::<web_sys::Element>() {
                                    let rect = el.get_bounding_client_rect();
                                    if rect.top() - wh <= 40.0 {
                                        let _ = el.class_list().add_1("fade-in-element");
                                    }
                                }
                            }
                        }
                    }
                }
            });

            move || {
                let _ = doc.remove_event_listener_with_callback(
                    "scroll",
                    check_scroll.as_ref().unchecked_ref(),
                );
                let _ = doc.remove_event_listener_with_callback(
                    "scroll",
                    handle_animations.as_ref().unchecked_ref(),
                );
                drop(check_scroll);
                drop(handle_animations);
                drop(timeout);
            }
        }, ());
    }

    let scroll_to_top = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if let Some(win) = window() {
            win.scroll_to_with_x_and_y(0.0, 0.0);
        }
    });

    html! {
        <>
            <Navbar />
            <main>
                <Carousel />

                // Trust bar under hero
                <div class="trust-bar">
                    <div class="container">
                        <div class="trust-bar-inner">
                            <div class="trust-bar-item">
                                <div class="trust-icon"><i class="bi bi-patch-check-fill"></i></div>
                                <span>{"Certificados por el Dept. de Salud PR"}</span>
                            </div>
                            <div class="trust-divider"></div>
                            <div class="trust-bar-item">
                                <div class="trust-icon"><i class="bi bi-shield-fill-check"></i></div>
                                <span>{"Proveedor Certificado de Medicare"}</span>
                            </div>
                            <div class="trust-divider"></div>
                            <div class="trust-bar-item">
                                <div class="trust-icon"><i class="bi bi-house-heart-fill"></i></div>
                                <span>{"Agencia Sin Fines de Lucro"}</span>
                            </div>
                            <div class="trust-divider"></div>
                            <div class="trust-bar-item">
                                <div class="trust-icon"><i class="bi bi-geo-alt-fill"></i></div>
                                <span>{"Servicio en 42 Municipios"}</span>
                            </div>
                        </div>
                    </div>
                </div>

                <Services />
                <Requirements />
                <About />
            </main>
            <Footer />

            // WhatsApp FAB
            <a
                href="https://wa.me/17878342295"
                class="whatsapp-fab"
                target="_blank"
                rel="noopener noreferrer"
                aria-label="Contactar por WhatsApp"
                title="Contáctanos por WhatsApp"
            >
                <i class="bi bi-whatsapp"></i>
            </a>

            // Scroll-to-top button
            if *show_scroll_button {
                <button
                    class="scroll-to-top"
                    onclick={scroll_to_top}
                    aria-label="Volver arriba"
                >
                    {"↑"}
                </button>
            }
        </>
    }
}
