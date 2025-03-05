use yew::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let show_menu = use_state(|| false);
    
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
                        if href.starts_with("#") {
                            let section_id = href.strip_prefix("#").unwrap_or_default();
                            if let Some(window) = window() {
                                if let Some(document) = window.document() {
                                    if let Ok(Some(section)) = document.query_selector(&format!("#{}", section_id)) {
                                        section.scroll_into_view();
                                        
                                        // Close mobile menu if open
                                        show_menu.set(false);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    };
    
    html! {
        <nav class="navbar navbar-expand-lg">
            <div class="container">
                <a class="navbar-brand" href="#">
                    <img class="navbar-logo" src="images/logo.png" alt="First Home Care Center" />
                    <span class="brand-text">{"First Home Care Center"}</span>
                </a>
                
                <button 
                    class="navbar-toggler" 
                    type="button" 
                    aria-controls="navbarNav" 
                    aria-expanded="false" 
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
