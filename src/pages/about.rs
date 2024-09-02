use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div>
            <h1>{"About us"}</h1>
            <p>{"This is the About page of our Yew router example."}</p>
        </div>
    }
}
