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
    
    // Handle animation on page load and scroll
    {
        let show_button = show_scroll_button.clone();
        
        use_effect_with_deps(move |_| {
            let window = window().expect("window not available");
            let document = window.document().expect("document not available");
            
            // Create closure for handling scroll to update button visibility
            let check_scroll = {
                let window = window.clone();
                let show_button = show_button.clone();
                
                Closure::wrap(Box::new(move |_: web_sys::Event| {
                    let scroll_top = window.scroll_y().unwrap_or(0.0);
                    show_button.set(scroll_top > 300.0);
                }) as Box<dyn FnMut(web_sys::Event)>)
            };
            
            // Add event listener for button visibility
            document.add_event_listener_with_callback(
                "scroll", 
                check_scroll.as_ref().unchecked_ref()
            ).expect("Failed to add scroll event listener");
            
            // Create closure for handling animations on scroll
            let handle_animations = {
                let window = window.clone();
                
                Closure::wrap(Box::new(move |_: web_sys::Event| {
                    if let Some(document) = window.document() {
                        if let Ok(elements) = document.query_selector_all(".animate-on-scroll") {
                            let window_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);
                            
                            for i in 0..elements.length() {
                                if let Some(node) = elements.get(i) {
                                    // Cast Node to Element to access Element-specific methods
                                    if let Some(element) = node.dyn_ref::<web_sys::Element>() {
                                        let rect = element.get_bounding_client_rect();
                                        let position_from_top = rect.top();
                                        
                                        if position_from_top - window_height <= 0.0 {
                                            let _ = element.class_list().add_1("fade-in-element");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(web_sys::Event)>)
            };
            
            // Add event listener for animations
            document.add_event_listener_with_callback(
                "scroll", 
                handle_animations.as_ref().unchecked_ref()
            ).expect("Failed to add animation scroll event listener");
            
            // Check elements in viewport on page load
            let window_clone = window.clone();
            let timeout = Timeout::new(100, move || {
                // Manually trigger animation check
                if let Some(document) = window_clone.document() {
                    if let Ok(elements) = document.query_selector_all(".animate-on-scroll") {
                        let window_height = window_clone.inner_height().unwrap().as_f64().unwrap_or(0.0);
                        
                        for i in 0..elements.length() {
                            if let Some(node) = elements.get(i) {
                                // Cast Node to Element to access Element-specific methods
                                if let Some(element) = node.dyn_ref::<web_sys::Element>() {
                                    let rect = element.get_bounding_client_rect();
                                    let position_from_top = rect.top();
                                    
                                    if position_from_top - window_height <= 0.0 {
                                        let _ = element.class_list().add_1("fade-in-element");
                                    }
                                }
                            }
                        }
                    }
                }
            });
            
            // Cleanup when component unmounts
            move || {
                // Remove event listeners
                let _ = document.remove_event_listener_with_callback(
                    "scroll", 
                    check_scroll.as_ref().unchecked_ref()
                );
                
                let _ = document.remove_event_listener_with_callback(
                    "scroll", 
                    handle_animations.as_ref().unchecked_ref()
                );
                
                // Keep closures alive for the lifetime of the component
                drop(check_scroll);
                drop(handle_animations);
                drop(timeout);
            }
        }, ());
    }
    
    // Handle scroll to top button click
    let scroll_to_top = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        if let Some(window) = window() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
    });

    html! {
        <>
            <Navbar />
            <main>
                <Carousel />
                <Services />
                <Requirements />
                <About />
            </main>
            <Footer />
            
            // Scroll to top button
            if *show_scroll_button {
                <button 
                    class="scroll-to-top" 
                    onclick={scroll_to_top}
                    aria-label="Scroll to top"
                >
                    {"↑"}
                </button>
            }
        </>
    }
}
