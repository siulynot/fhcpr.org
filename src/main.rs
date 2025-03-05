mod app;
mod components;
mod pages;

use app::App;

fn main() {
    // Initialize app
    yew::Renderer::<App>::new().render();
}
