use abhinandhs_deno_dev::switch;
use abhinandhs_deno_dev::Route;
use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html!(
     <HashRouter>
        <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
    </HashRouter>
    )
}
