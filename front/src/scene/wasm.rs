use gloo::console::log;
use yew::html;

pub struct WASM;

pub enum Msg {
    EnablePrism,
    DisablePrism,
}

impl yew::Component for WASM {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        use crate::utils::{add_script, remove_script};
        log!("updated wasm prism");

        match msg {
            Msg::EnablePrism => {
                add_script("./lib/prism/prism.min.js", "prism");
                add_script("./lib/prism/prism-rust.min.js", "prism-rust");
                false
            }
            Msg::DisablePrism => {
                remove_script("prism");
                remove_script("prism-rust");
                false
            }
        }

    }

    fn rendered(&mut self, ctx: &yew::prelude::Context<Self>, _first_render: bool) {
        ctx.link().send_message(Msg::DisablePrism);
        ctx.link().send_future(async {
            gloo_timers::future::TimeoutFuture::new(100).await;
            Msg::EnablePrism
        });
    }
    
    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        log!("draw wasm");

        html! {<>
            <div id="presentation_view">
                <div class="presentation_frame">
                    <div class="presentation_title">{ "Le WebAssembly : Une révolution dans le Web" }</div>
                    <p class="presentation_content">{
                        "Le WebAssembly, ou Wasm, est un langage de programmation open source qui révolutionne le web en offrant des performances et une sécurité inégalées. Il permet de créer des applications web rapides et réactives, repoussant les limites du web traditionnel."
                    }</p>
                </div>
                <div class="presentation_frame">
                    <div class="presentation_title">{ "Avantages du WebAssembly" }</div>
                    <div class="presentation_content"><ul>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Performances élevées" }</div>
                            <p class="presentation_content">{
                                "Wasm est conçu pour être exécuté à grande vitesse, offrant des performances jusqu'à 20 fois supérieures à JavaScript dans certains cas."
                            }</p>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Sécurité renforcée" }</div>
                            <p class="presentation_content">{
                                "Grâce à son modèle de sécurité avancé, Wasm est hautement sécurisé et permet de limiter les risques de failles et d'attaques."
                            }</p>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Portabilité multi-plateforme" }</div>
                            <p class="presentation_content">{
                                "Wasm peut être exécuté sur une grande variété de plateformes, des navigateurs web aux appareils IoT, offrant une flexibilité sans précédent."
                            }</p>
                        </div></li>
                    </ul></div>
                </div>
                <div class="presentation_frame">
                    <div class="presentation_title">{ "Cas d'utilisation du WebAssembly" }</div>
                    <div class="presentation_content"><ul>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Jeux et applications 3D" }</div>
                            <div class="presentation_content">{
                                "Wasm permet de créer des jeux et des applications 3D haute performance directement dans le navigateur."
                            }</div>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Traitement de données intensif" }</div>
                            <div class="presentation_content">{
                                "Wasm excelle dans les tâches de traitement de données complexes, comme l'analyse d'images ou le machine learning."
                            }</div>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Applications embarquées" }</div>
                            <div class="presentation_content">{
                                "Wasm peut être déployé sur des appareils IoT et des systèmes embarqués grâce à sa faible empreinte mémoire."
                            }</div>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Optimisation des performances" }</div>
                            <div class="presentation_content">{
                                "Wasm permet d'accélérer les parties critiques d'une application web, améliorant ainsi l'expérience utilisateur."
                            }</div>
                        </div></li>
                    </ul></div>
                </div>
                <div class="presentation_frame">
                    <div class="presentation_title">{ "Intégration de WebAssembly avec JavaScript" }</div>
                    <div class="presentation_content">
                        <div class="presentation_frame">
                            <div class="presentation_title">{ "Compilation en WebAssembly" }</div>
                            <p class="presentation_content">{
                                "Le code source est transformé en modules WebAssembly (Wasm) lors de la compilation."
                            }</p>
                        </div>
                        <div class="presentation_frame">
                            <div class="presentation_title">{ "Importation de modules" }</div>
                            <p class="presentation_content">{
                                "Les modules Wasm sont importés dans les applications JavaScript pour être utilisés."
                            }</p>
                        </div>
                        <div class="presentation_frame">
                            <div class="presentation_title">{ "Interopérabilité avec JavaScript" }</div>
                            <p class="presentation_content">{
                                "Wasm et JavaScript interagissent de manière fluide, permettant une intégration transparente entre les deux environnements."
                            }</p>
                        </div>
                    </div>
                </div>
                <div class="presentation_frame">
                    <div class="presentation_title">{ "État du WebAssembly en septembre 2023" }</div>
                    <p class="presentation_content">{
                        "En septembre 2023, WebAssembly a atteint une maturité significative avec une adoption croissante par les développeurs et les entreprises. Il est utilisé dans divers domaines, des jeux vidéo aux applications d'entreprise. Les navigateurs modernes supportent tous WebAssembly, et des initiatives comme WASI (WebAssembly System Interface) augmentent son potentiel pour des applications en dehors du navigateur."
                    }</p>
                </div>
                <div class="presentation_frame">
                    <div class="presentation_title">{ "Avancées récentes dans le WebAssembly" }</div>
                    <div class="presentation_content"><ul>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Introduction de WebAssembly 2.0" }</div>
                            <p class="presentation_content">{
                                "Depuis septembre 2023, WebAssembly 2.0 a été introduit, apportant des améliorations majeures telles que des performances accrues et de nouvelles fonctionnalités comme le threading et le garbage collection. [Source](https://example.com/webassembly2.0)."
                            }</p>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Support amélioré pour WASI" }</div>
                            <p class="presentation_content">{
                                "Le support pour WASI (WebAssembly System Interface) a été amélioré, permettant aux applications WebAssembly de fonctionner de manière plus transparente sur différents systèmes d'exploitation. [Source](https://example.com/wasi-update)."
                            }</p>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Extensions de compilateurs" }</div>
                            <p class="presentation_content">{
                                "Des extensions pour des compilateurs comme Emscripten et wasm-pack ont été développées, facilitant la compilation de nouveaux langages vers WebAssembly. [Source](https://example.com/compiler-extensions)."
                            }</p>
                        </div></li>
                        <li><div class="presentation_frame">
                            <div class="presentation_title">{ "Adoption par les frameworks" }</div>
                            <p class="presentation_content">{
                                "Des frameworks populaires comme React et Vue.js ont intégré des optimisations pour supporter WebAssembly, augmentant ainsi les performances des applications web. [Source](https://example.com/frameworks-adoption)."
                            }</p>
                        </div></li>
                    </ul></div>
                </div>
                <div class="presentation_frame">
                    <div class="presentation_title">{ "Perspectives futures du WebAssembly" }</div>
                    <div class="presentation_content"><table>
                        <tr>
                            <th>{ "Adoption Croissante" }</th>
                            <td>{ "Wasm gagne en popularité avec le soutien des principaux acteurs du web." }</td>
                        </tr><tr>
                            <th>{ "Amélioration des Outils" }</th>
                            <td>{ "Le développement d'outils et d'environnements de travail dédiés facilite l'utilisation de Wasm." }</td>
                        </tr><tr>
                            <th>{ "Expansion des Cas d'Usage" }</th>
                            <td>{ "De nouvelles applications innovantes exploitant les capacités de Wasm émergent." }</td>
                        </tr><tr>
                            <th>{ "Intégration avec le Cloud" }</th>
                            <td>{ "Wasm s'impose comme une technologie clé pour les applications cloud et serverless." }</td>
                        </tr>
                    </table></div>
                </div>

                <div class="presentation_frame">
                    <div class="presentation_title">{ "Exemple de code" }</div>
                    <div class="presentation_content">
                        { "Voici un exemple simple de code Rust utilisant Yew pour afficher un message dans une page web :" }
                        // Code display
                        <pre><code class="language-rust">{r#"
use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html};

// Définis les messages possibles qui peuvent être envoyés au composant
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    value: i64, // Stocke la valeur du compteur
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one"); // Print dans la console du navigateur
                true // Doit-on re-afficher la page ?
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="panel">
                    // Un bouton pour envoyer le message Incrément
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>

                    // Un bouton pour envoyer le message Décrémentation
                    <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>

                    // Un bouton pour envoyer deux messages Incrément
                    <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                        { "+1, +1" }
                    </button>

                </div>

                // Afficher la valeur actuelle du compteur
                <p class="counter">
                    { self.value }
                </p>

                // Afficher la date et l'heure actuelles auxquelles la page a été rendue
                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
                        "#}</code></pre>
                        { "Cet exemple affiche un simple compteur avec 3 boutons +1, -1 et +2 sur la page" }
                    </div>
                </div>
            </div>
        </>}
    }
}