use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <nav>
                <a href="#" class="brand">
                    <span>{ "Blog" }</span>
                </a>
                <div class=""menu>
                    <a href="#" class="button icon-puzzle">{ "About" }</a>
                </div>
            </nav>
            <p>{""}</p>
            <p>{""}</p>
            <p>{""}</p>
            <h1>{ "Hello, world!" }</h1>
        </>
    }
}
