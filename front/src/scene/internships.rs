use gloo::console::log;
use yew::html;

pub struct Internships;

pub enum Message {
    Pop,
}

impl yew::Component for Internships {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        ctx.link().send_message(Message::Pop);
        Self
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        use wasm_bindgen::JsCast as _;

        match msg {
            Message::Pop => {
                log!("pop");
                ctx.link().send_future(async {
                    gloo_timers::future::TimeoutFuture::new(5000).await;
                    Self::Message::Pop
                }); 
                return false;
            }
        }

        true
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {<>
            <div class="wasm_frame">
                <div class="wasm_title">{ "Réalisation d'un portfolio" }</div>
                <p class="wasm_content">{
                    "Réalisation d'un portfolio de présentation en HTML / CSS / JS puis en Next.js 14 pour apprendre les fondamentaux. Déploiement de ce portfolio sur https://stage.asf-web.fr/hugo/ et bowarc.ovh."
                }</p>
            </div>
            <div class="wasm_frame">
                <div class="wasm_title">{ "Observation des performances" }</div>
                <p class="wasm_content">{
                    "Observation des performances avec Google Lighthouse, retour sur les fichiers du portfolio pour améliorer le SEO et les bonnes pratiques."
                }</p>
            </div>
            <div class="wasm_frame">
                <div class="wasm_title">{ "Conception et réalisation d'un serveur de stockage" }</div>
                <div class="wasm_content">
                    <div class="wasm_frame">
                        <div class="wasm_title">{ "Backend en Rust" }</div>
                        <p class="wasm_content">{
                            "Conception et réalisation d'un serveur de stockage en Rust avec une API JSON et un encodage base64 pour les fichiers, compression Brotli pour le stockage."
                        }</p>
                    </div>
                    <div class="wasm_frame">
                        <div class="wasm_title">{ "Frontend en WASM" }</div>
                        <p class="wasm_content">{
                            "Développement de la partie front-end en WebAssembly."
                        }</p>
                    </div>
                </div>
            </div>
            <div class="wasm_frame">
                <div class="wasm_title">{ "Résolution de problèmes de CORS" }</div>
                <p class="wasm_content">{
                    "Résolution de problèmes de CORS."
                }</p>
            </div>
            <div class="wasm_frame">
                <div class="wasm_title">{ "Documentation et gestion des erreurs" }</div>
                <p class="wasm_content">{
                    "Documentation avec exemples, logging et gestion des potentielles erreurs."
                }</p>
            </div>
            <div class="wasm_frame">
                <div class="wasm_title">{ "Publication et versionning" }</div>
                <p class="wasm_content">{
                    "Publication et versionning à l'aide de git / GitHub."
                }</p>
            </div>
            <div class="wasm_frame">
                <div class="wasm_title">{ "Installation et configuration d'un VPS Linux Debian" }</div>
                <p class="wasm_content">{
                    "Installation et configuration d'un VPS Linux Debian."
                }</p>
            </div>
        </>}
    }
}
