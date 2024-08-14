use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;
use crate::pages::home::Home;
use crate::pages::about::About;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let nav_classes = classes!("nav");
    let link_classes = use_state(|| classes!("nav-link"));

    html! {
        <BrowserRouter>
            <nav class={nav_classes}>
                <Link<Route> to={Route::Home} classes={(*link_classes).clone()}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::About} classes={(*link_classes).clone()}>{ "About" }</Link<Route>>
            </nav>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}
