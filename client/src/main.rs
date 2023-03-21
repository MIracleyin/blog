mod app;
mod components;
mod fetch;
mod models;

use yew::prelude::*;

fn main() {
    yew::Renderer::<app::App>::new().render();
}

