use gloo::console::log;
use yew::html;

pub struct WASM;

impl yew::Component for WASM {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &yew::prelude::Context<Self>, _msg: Self::Message) -> bool {
        log!("update wasm");

        enable_prism();

        false
    }

    fn rendered(&mut self, ctx: &yew::prelude::Context<Self>, _first_render: bool) {
        ctx.link().send_future(async {
            disable_prism();

            // gloo_timers::future::TimeoutFuture::new(100).await;
        }); // Implicit () which is the message type of this component
    }
    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        log!("draw wasm");

        html! {<>
                    <div id="wasm_view">
                        <h1>{"Introduction au WebAssembly (Wasm)"}</h1>
                        <p>{"WebAssembly (Wasm) est un format de fichier binaire et un langage de bas niveau qui exécute des instructions dans les navigateurs web. Voici quelques points clés à retenir à son sujet :"}</p>
                        <ul>
                            <li>{"Performance : Wasm permet d'exécuter du code à une vitesse proche de celle du langage natif, ce qui le rend idéal pour les applications web exigeantes en termes de performances."}</li>
                            <li>{"Langages supportés : Wasm peut être compilé à partir de nombreux langages de programmation, notamment C/C++, Rust, et bien d'autres."}</li>
                            <li>{"Portabilité : Une fois compilé en Wasm, le code peut être exécuté dans n'importe quel navigateur web moderne, ainsi que dans des environnements non navigateur comme Node.js."}</li>
                            <li>{"Sécurité : Wasm est conçu avec un modèle de sécurité robuste qui isole le code exécuté dans un environnement sécurisé, réduisant ainsi les risques de failles de sécurité."}</li>
                        </ul>
                        <h2>{"Utilisation de Wasm avec Yew"}</h2>
                        <p>{"Yew est un framework web écrit en Rust qui facilite l'utilisation de WebAssembly pour la construction d'interfaces utilisateur web. Voici quelques avantages de l'utilisation de Yew avec Wasm :"}</p>
                        <ul>
                            <li>{"Productivité : Yew offre un environnement de développement familier pour les développeurs Rust, ce qui leur permet de tirer parti de la sécurité et de la performance de Rust tout en construisant des applications web."}</li>
                            <li>{"Interopérabilité : Yew facilite l'intégration de code JavaScript existant avec des composants écrits en Rust et compilés en Wasm, ce qui permet aux développeurs de combiner les forces des deux langages."}</li>
                            <li>{"Performances : En utilisant Wasm avec Yew, les développeurs peuvent créer des applications web rapides et réactives, capables de rivaliser avec des applications natives en termes de performances."}</li>
                        </ul>
                        <h2>{"Exemple de code"}</h2>
                        <p>{"Voici un exemple simple de code Rust utilisant Yew pour afficher un message dans une page web :"}</p>
                        <pre>
        //                     <code class="language-rust">
        //                     // Code display
        //                         {r#"
        // use gloo::console;
        // use js_sys::Date;
        // use yew::{html, Component, Context, Html};

        // // Définis les messages possibles qui peuvent être envoyés au composant
        // pub enum Msg {
        //     Increment,
        //     Decrement,
        // }

        // pub struct App {
        //     value: i64, // Stocke la valeur du compteur
        // }

        // impl Component for App {
        //     type Message = Msg;
        //     type Properties = ();

        //     fn create(_ctx: &Context<Self>) -> Self {
        //         Self { value: 0 }
        //     }

        //     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        //         match msg {
        //             Msg::Increment => {
        //                 self.value += 1;
        //                 console::log!("plus one"); // Print dans la console du navigateur
        //                 true // Doit-on re-afficher la page ?
        //             }
        //             Msg::Decrement => {
        //                 self.value -= 1;
        //                 console::log!("minus one");
        //                 true
        //             }
        //         }
        //     }

        //     fn view(&self, ctx: &Context<Self>) -> Html {
        //         html! {
        //             <div>
        //                 <div class="panel">
        //                     // Un bouton pour envoyer le message Incrément
        //                     <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
        //                         { "+1" }
        //                     </button>

        //                     // Un bouton pour envoyer le message Décrémentation
        //                     <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
        //                         { "-1" }
        //                     </button>

        //                     // Un bouton pour envoyer deux messages Incrément
        //                     <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
        //                         { "+1, +1" }
        //                     </button>

        //                 </div>

        //                 // Afficher la valeur actuelle du compteur
        //                 <p class="counter">
        //                     { self.value }
        //                 </p>

        //                 // Afficher la date et l'heure actuelles auxquelles la page a été rendue
        //                 <p class="footer">
        //                     { "Rendered: " }
        //                     { String::from(Date::new_0().to_string()) }
        //                 </p>
        //             </div>
        //         }
        //     }
        // }

        // fn main() {
        //     yew::Renderer::<App>::new().render();
        // }
        //                         "#}
        //                     </code>
                        </pre>
                        <p>{"Cet exemple affiche un simple compteur avec 3 boutons +1, -1 et +2 sur la page"}</p>

                        <div class="frame">
                            <div class="title">{
                                "Le WebAssembly : Une Révolution dans le Web"
                            }</div>
                            <p class="content">{
                                "Le WebAssembly, ou Wasm, est un langage de programmation open source qui révolutionne le web en offrant des performances et une sécurité inégalées. Il permet de créer des applications web rapides et réactives, repoussant les limites du web traditionnel."
                            }</p>
                        </div>
                        <div class="frame">
                            <div class="title">{
                                "Avantages du WebAssembly"
                            }</div>
                            <div class="content">
                                <div class="frame">
                                    <div class="title">{
                                        "Performances élevées"
                                    }</div>
                                    <p class="content">{
                                        "Wasm est conçu pour être exécuté à grande vitesse, offrant des performances jusqu'à 20 fois supérieures à JavaScript dans certains cas."
                                    }</p>
                                </div>
                                <div class="frame">
                                    <div class="title">{
                                        "Sécurité renforcée"
                                    }</div>
                                    <p class="content">{
                                        "Grâce à son modèle de sécurité avancé, Wasm est hautement sécurisé et permet de limiter les risques de failles et d'attaques."
                                    }</p>
                                </div>
                                <div class="frame">
                                    <div class="title">{
                                        "Portabilité multi-plateforme"
                                    }</div>
                                    <p class="content">{
                                        "Portabilité multi-plateforme, Wasm peut être exécuté sur une grande variété de plateformes, des navigateurs web aux appareils IoT, offrant une flexibilité sans précédent."
                                    }</p>
                                </div>
                            </div>
                        </div>
                        <div class="frame">
                            <div class="title">{
                                "Cas d'utilisation du Webassembly"
                            }</div>
                            <div class="content">
                                <ul>
                                    <li><div class="frame">
                                        <div class="title">
                                            <span class="numbered-square">{1}</span>
                                        {
                                            "Jeux et applications 3D"
                                        }</div>
                                        <div class="content">{
                                            "Wasm permet de créer des jeux et des applications 3D haute performance directement dans le navigateur."
                                        }</div>
                                    </div></li>
                                    <li><div class="frame">
                                        <div class="title">{
                                            "Traitement de données intensif"
                                        }</div>
                                        <div class="content">{
                                            "Wasm excelle dans les tâches de traitement de données complexes, comme l'analyse d'images ou le machine learning."
                                        }</div>
                                    </div></li>
                                    <li><div class="frame">
                                        <div class="title">{
                                            "Applications embarquées"
                                        }</div>
                                        <div class="content">{
                                            "Wasm peut être déployé sur des appareils IoT et des systèmes embarqués grâce à sa faible empreinte mémoire."
                                        }</div>
                                    </div></li>
                                    <li><div class="frame">
                                        <div class="title">{
                                            "Optimisation des performances"
                                        }</div>
                                        <div class="content">{
                                            "Wasm permet d'accélérer les parties critiques d'une application web, améliorant ainsi l'expérience utilisateur."
                                        }</div>
                                    </div></li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </>}
    }
}

fn disable_prism() {
    use crate::utils::remove_script;
    remove_script("prism");
    remove_script("prism-rust");
}

fn enable_prism() {
    use crate::utils::add_script;
    add_script("./lib/prism/prism.min.js", "prism");
    add_script("./lib/prism/prism-rust.min.js", "prism-rust");
}
