use yew::html;

pub struct Contact;

impl yew::Component for Contact {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {<>
            <div>
                <h1>{"Contact Me"}</h1>
                <p>{"You can contact me through the following channels:"}</p>
                <ul>
                    <li>{"GitHub: "}
                        <a href="https://github.com/Bowarc" target="_blank">{"Bowarc"}</a>{" & "}
                        <a href="https://github.com/HugoLz" target="_blank">{"HugoLz"}</a>
                    </li>
                    <li>{"Email: "}
                        <a href="mailto:Bowarc@proton.me">{"Bowarc@proton.me"}</a>{" & "}
                        <a href="mailto:hugo.lezoualch@proton.me">{"hugo.lezoualch@proton.me"}</a>
                    </li>
                </ul>
            </div>
        </>}
    }
}
