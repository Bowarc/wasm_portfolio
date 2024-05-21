use gloo::console::log;
use yew::html;

pub struct Internships;

pub enum Message {
    Pop,
}

impl yew::Component for Internships {
    type Message = ();

    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        // ctx.link().send_message(Message::Pop);
        Self
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        log!("updated internship zoom");
        // use wasm_bindgen::JsCast as _;

        // match msg {
        //     Message::Pop => {
        //         log!("pop");
        //         ctx.link().send_future(async {
        //             gloo_timers::future::TimeoutFuture::new(5000).await;
        //             Self::Message::Pop
        //         });
        //         false
        //     }
        // }
        false
    }

    fn rendered(&mut self, ctx: &yew::prelude::Context<Self>, _first_render: bool) {
        ctx.link().send_future(async {
            disable_zoom();

            gloo_timers::future::TimeoutFuture::new(100).await;

            enable_zoom();
        }); // Implicit () which is the message type of this component
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {<>
            <div class="presentation_frame">
                <div class="presentation_title">{ "Projets de l'année 2023-2024" }</div>
                <div class="presentation_content">
                    <ul>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Jeu d'échec en multijoueur." }</div>
                            <div class="presentation_content">
                                { "Un jeu d'échec en multijoueur codé en Rust" }
                                // <div class="zoomable_element" style="background-image: url('/resources/storage_server.drawio200px.png');"></div>
                            </div>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Réalisation d'un portfolio basique" }</div>
                            <p class="presentation_content">
                                { "Réalisation d'un portfolio de présentation en HTML / CSS / JS puis en Next.js 14 pour apprendre les fondamentaux. " }
                                { "Déploiement de ce portfolio sur " } 
                                <a href="https://stage.asf-web.fr/hugo/" class="link">{ "https://stage.asf-web.fr/hugo/"} </a>
                                { " et " }
                                <a href="https://bowarc.ovh/old" class="link">{ "https://bowarc.ovh/old." }</a>
                            </p>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Serveur de stockage et de compression" }</div>
                            <div class="presentation_content">
                                { "Serveur de stockage et de compression avec une api en JSON" }
                                // <div class="zoomable_element" style="background-image: url('/resources/storage_server.drawio200px.png');"></div>
                            </div>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Blackjack" }</div>
                            <div class="presentation_content">{
                                ""
                            }</div>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Centrale inertielle + flightgear" }</div>
                            <div class="presentation_content">{
                                ""
                            }</div>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Scanneur à addresse mémoire" }</div>
                            <div class="presentation_content">{
                                ""
                            }</div>
                        </div></li>
                    </ul>
                </div>
            </div>
        </>}
    }
}

fn disable_zoom() {
    use crate::utils::remove_script;
    remove_script("zoom");
}

fn enable_zoom() {
    use crate::utils::add_script;
    add_script("./lib/zoom/zoom.js", "zoom");
}
