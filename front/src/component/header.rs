use yew::html;

pub struct Header;

impl yew::Component for Header {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {
            <div id="header">
                <a class="header_item" href="http://github.com/Bowarc">
                    <img src="resources/github.webp" alt="Github icon" class="icon"/>
                </a>
            </div>
        }
    }
}
