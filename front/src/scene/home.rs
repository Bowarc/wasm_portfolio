use yew::html;
use crate::component;

pub struct Home;

impl yew::Component for Home {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {<>
            // Under developpement warning
            <p class="warning">{
                "This is still under construction and nowhere close to done"
            }</p>
            <p id="description">
                <p>
                    {"Développeur Autodidacte de " }
                    <component::Age/>
                    { ", spécialisé dans le développement logiciel et backend."}
                </p>
                <p>{"J'ai commencé avec Python avant de me concentrer principalement sur Rust, un langage dans lequel je vois un potentiel considérable pour ses performances et sa sécurité."}</p>
                <p>{"Je suis familier avec les environnements Windows et Linux, ce qui me permet de concevoir des solutions robustes et polyvalentes."}</p>
                <p>{"Mon expertise s'étend à la construction et à la maintenance d'applications de bureau ainsi qu'à la mise en place de solutions côté serveur."}</p>
                <p>
                    {"En outre, je suis compétent dans le développement front-end, notamment avec "}
                    <a href="https://yew.rs/" class="link">{"Yew"}</a>
                    {" via "}
                    <a href="https://webassembly.org/" class="link">{"WebAssembly"}</a>
                    {", ce qui me permet de créer des expériences utilisateur riches et réactives."}
                </p>
                <p>{"J'ai eu l'occasion de travailler sur divers projets, dont [insérer exemple notable ou quantifiable ici], démontrant ma capacité à livrer des solutions de haute qualité dans des environnements variés."}</p>
                <p>{"Je suis passionné par l'innovation technologique et je m'efforce constamment d'approfondir mes compétences pour relever de nouveaux défis."}</p>
            </p>
        </>}
    }
}
