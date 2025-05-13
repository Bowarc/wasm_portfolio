use yew::{function_component, html, Html};

#[function_component]
pub fn NotFound() -> Html {
    html! {<div class="not-found">
        <h1>{ "404" }</h1>
        <p>
            { "The page you requested has not been found, click " }
            <a href="/">{ "here"}</a>
            { " to go back to the main page." }
        </p>
    </div>}
}
