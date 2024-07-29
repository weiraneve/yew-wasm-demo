use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Welcome to the Home Page"}</h1>
            <p>{"This is a simple Yew router example."}</p>
        </div>
    }
}
