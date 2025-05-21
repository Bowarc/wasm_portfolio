use yew::{function_component, Html, html};

#[function_component]
pub fn Contact() -> Html {
    if let Some(nav) = yew_router::hooks::use_navigator() {
        nav.replace(&crate::Route::Contact)
    }else{
        error!("Failed to retrieve the navigator")
    }

    html! {<div id="contact">
        <h1>{"Contact Me"}</h1>
        <p>{"You can reach me at: "}</p>
        <table>
            <tr>
                <th>{
                    "Email:"
                }</th>
                <td>
                    <a href="mailto:contact@bowarc.ovh">{"contact@bowarc.ovh"}</a>
                </td>
            </tr>
        </table>
    </div>}
}
