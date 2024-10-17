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
            <div class="contact">
                <h1>{"Contact Me"}</h1>
                <p>{"You can reach me at: "}</p>
                <table>
                    <tr>
                        <th>{
                            "Email:"
                        }</th>
                        <td>
                            <a href="mailto:bowarc@proton.me">{"Bowarc@proton.me"}</a>
                            <br />
                            <a href="mailto:hugo.lezoualch@proton.me">{"hugo.lezoualch@proton.me"}</a>
                        </td>
                    </tr>
                    <tr>
                        <th>{
                            "Discord:"
                        }</th>
                        <td>{
                            "Bowarc"
                        }</td>
                    </tr>
                    <tr>
                        <th>{
                            "Twitter:"
                        }</th>
                        <td>{
                            "@Bowarc_"
                        }</td>
                    </tr>
                </table>
            </div>
        </>}
    }
}
