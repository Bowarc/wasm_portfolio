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
                // <h1>{"Introduction au WebAssembly (Wasm)"}</h1>
                // <p>{"WebAssembly (Wasm) est un format de fichier binaire et un langage de bas niveau qui exécute des instructions dans les navigateurs web. Voici quelques points clés à retenir à son sujet :"}</p>
                // <ul>
                //     <li>{"Performance : Wasm permet d'exécuter du code à une vitesse proche de celle du langage natif, ce qui le rend idéal pour les applications web exigeantes en termes de performances."}</li>
                //     <li>{"Langages supportés : Wasm peut être compilé à partir de nombreux langages de programmation, notamment C/C++, Rust, et bien d'autres."}</li>
                //     <li>{"Portabilité : Une fois compilé en Wasm, le code peut être exécuté dans n'importe quel navigateur web moderne, ainsi que dans des environnements non navigateur comme Node.js."}</li>
                //     <li>{"Sécurité : Wasm est conçu avec un modèle de sécurité robuste qui isole le code exécuté dans un environnement sécurisé, réduisant ainsi les risques de failles de sécurité."}</li>
                // </ul>
                // <h2>{"Utilisation de Wasm avec Yew"}</h2>
                // <p>{"Yew est un framework web écrit en Rust qui facilite l'utilisation de WebAssembly pour la construction d'interfaces utilisateur web. Voici quelques avantages de l'utilisation de Yew avec Wasm :"}</p>
                // <ul>
                //     <li>{"Productivité : Yew offre un environnement de développement familier pour les développeurs Rust, ce qui leur permet de tirer parti de la sécurité et de la performance de Rust tout en construisant des applications web."}</li>
                //     <li>{"Interopérabilité : Yew facilite l'intégration de code JavaScript existant avec des composants écrits en Rust et compilés en Wasm, ce qui permet aux développeurs de combiner les forces des deux langages."}</li>
                //     <li>{"Performances : En utilisant Wasm avec Yew, les développeurs peuvent créer des applications web rapides et réactives, capables de rivaliser avec des applications natives en termes de performances."}</li>
                // </ul>

                <div class="wasm_frame">
                    <div class="wasm_title">{ "Le WebAssembly : Une Révolution dans le Web" }</div>
                    <p class="wasm_content">{
                        "Le WebAssembly, ou Wasm, est un langage de programmation open source qui révolutionne le web en offrant des performances et une sécurité inégalées. Il permet de créer des applications web rapides et réactives, repoussant les limites du web traditionnel."
                    }</p>
                </div>
                <div class="wasm_frame">
                    <div class="wasm_title">{ "Avantages du WebAssembly" }</div>
                    <div class="wasm_content">
                        <div class="wasm_frame">
                            <div class="wasm_title">{ "Performances élevées" }</div>
                            <p class="wasm_content">{
                                "Wasm est conçu pour être exécuté à grande vitesse, offrant des performances jusqu'à 20 fois supérieures à JavaScript dans certains cas."
                            }</p>
                        </div>
                        <div class="wasm_frame">
                            <div class="wasm_title">{ "Sécurité renforcée" }</div>
                            <p class="wasm_content">{
                                "Grâce à son modèle de sécurité avancé, Wasm est hautement sécurisé et permet de limiter les risques de failles et d'attaques."
                            }</p>
                        </div>
                        <div class="wasm_frame">
                            <div class="wasm_title">{ "Portabilité multi-plateforme" }</div>
                            <p class="wasm_content">{
                                "Portabilité multi-plateforme, Wasm peut être exécuté sur une grande variété de plateformes, des navigateurs web aux appareils IoT, offrant une flexibilité sans précédent."
                            }</p>
                        </div>
                    </div>
                </div>
                <div class="wasm_frame">
                    <div class="wasm_title">{ "Cas d'utilisation du Webassembly" }</div>
                    <div class="wasm_content">
                        <ul>
                            <li><div class="wasm_frame">
                                <div class="wasm_title">{ "Jeux et applications 3D" }</div>
                                <div class="wasm_content">{
                                    "Wasm permet de créer des jeux et des applications 3D haute performance directement dans le navigateur."
                                }</div>
                            </div></li>
                            <li><div class="wasm_frame">
                                <div class="wasm_title">{ "Traitement de données intensif" }</div>
                                <div class="wasm_content">{
                                    "Wasm excelle dans les tâches de traitement de données complexes, comme l'analyse d'images ou le machine learning."
                                }</div>
                            </div></li>
                            <li><div class="wasm_frame">
                                <div class="wasm_title">{ "Applications embarquées" }</div>
                                <div class="wasm_content">{
                                    "Wasm peut être déployé sur des appareils IoT et des systèmes embarqués grâce à sa faible empreinte mémoire."
                                }</div>
                            </div></li>
                            <li><div class="wasm_frame">
                                <div class="wasm_title">{ "Optimisation des performances" }</div>
                                <div class="wasm_content">{
                                    "Wasm permet d'accélérer les parties critiques d'une application web, améliorant ainsi l'expérience utilisateur."
                                }</div>
                            </div></li>
                        </ul>
                    </div>
                </div>
                <div class="wasm_frame">
                    <div class="wasm_title">{ "Languages et outils WebAssembly" }</div>
                    <div class="wasm_content"><ul>
                        <li><div class="wasm_frame">
                            <div class="wasm_title">{ "C/C++" }</div>
                            <div class="wasm_content">{
                                "C/C++ sont des langages de programmation puissants, largement utilisés pour leurs performances et leur contrôle précis de la mémoire. Leur compilation en WebAssembly à l'aide d'outils tels qu'Emscripten permet de créer des applications WebAssembly performantes et efficaces."
                            }</div>
                        </div></li>
                        <li><div class="wasm_frame">
                            <div class="wasm_title">{ "Rust" }</div>
                            <div class="wasm_content">{
                                "Rust est un langage moderne privilégiant performance et sécurité. Sa compilation vers WebAssembly via des outils comme wasm-pack offre des applications web performantes avec un contrôle précis de la mémoire et des garanties de sécurité."
                            }</div>
                        </div></li>
                        <li><div class="wasm_frame">
                            <div class="wasm_title">{ "JavaScript" }</div>
                            <div class="wasm_content">{
                                "JavaScript est un langage de programmation largement utilisé pour le développement web. Sa compilation en WebAssembly grâce à Javy et Wasmtime ouvre la voie à des applications hautement performantes, offrant ainsi une alternative pour des performances optimisées."
                            }</div>
                        </div></li>
                        <li><div class="wasm_frame">
                            <div class="wasm_title">{ "Go" }</div>
                            <div class="wasm_content">{
                                "Go est un langage moderne avec une concurrence native et une gestion automatique de la mémoire. Compilé en WebAssembly avec `GOARCH=wasm GOOS=js`, il garantit des performances élevées et une utilisation efficace des fonctionnalités de Go dans les applications WebAssembly."
                            }</div>
                        </div></li>
                        <li><div class="wasm_frame">
                            <div class="wasm_title">{ "AssemblyScript" }</div>
                            <div class="wasm_content">{
                                "Il s'agit d'un langage similaire à TypeScript mais conçu pour être compilé en WebAssembly, fournissant ainsi une alternative à JavaScript pour les performances optimisées."
                            }</div>
                        </div></li>
                        <li><div class="wasm_frame">
                            <div class="wasm_title">{ "Zig" }</div>
                            <div class="wasm_content">{
                                "Zig est un langage de programmation polyvalent conçu pour la robustesse, l'optimalité et la maintenabilité. Sa compilation en WebAssembly ouvre de nouvelles perspectives pour des applications web performantes et fiables, exploitant pleinement les capacités de Zig dans un environnement web."
                            }</div>
                        </div></li>
                    </ul></div>
                </div>

                <div class="wasm_frame">
                    <div class="wasm_title">{ "Intégration de WebAssembly avec JavaScript" }</div>
                    <div class="wasm_content">
                        <div class="wasm_frame">
                            <div class="wasm_title">{ "Compilation" }</div>
                            <p class="wasm_content">{
                                "Le code source est transformé en modules WebAssembly (Wasm) lors de la compilation."
                            }</p>
                        </div>
                        <div class="wasm_frame">
                            <div class="wasm_title">{ "Importation" }</div>
                            <p class="wasm_content">{
                                "Les modules Wasm sont importés dans les applications JavaScript pour être utilisés."
                            }</p>
                        </div>
                        <div class="wasm_frame">
                            <div class="wasm_title">{ "Interopérabilité" }</div>
                            <p class="wasm_content">{
                                "Wasm et JavaScript interagissent de manière fluide, permettant une intégration transparente entre les deux environnements."
                            }</p>
                        </div>
                    </div>
                </div>

                <div class="wasm_frame">
                    <div class="wasm_title">{ "Tendances et perspectives futures de WebAssembly" }</div>
                    <div class="wasm_content"><table>
                        <tr>
                            <th>{ "Adoption croissante" }</th>
                            <td>{ "Wasm gagne en popularité avec le soutien des principaux acteurs du web." }</td>
                        </tr><tr>
                            <th>{ "Amélioration des outils" }</th>
                            <td>{ "Le développement d'outils et d'environnements de travail dédiés facilite l'utilisation de Wasm." }</td>
                        </tr><tr>
                            <th>{ "Expansion des cas d'usage" }</th>
                            <td>{ "De nouvelles applications innovantes exploitant les capacités de Wasm émergent." }</td>
                        </tr><tr>
                            <th>{ "Intégration avec le cloud" }</th>
                            <td>{ "Wasm s'impose comme une technologie clé pour les applications cloud et serverless." }</td>
                        </tr>
                    </table></div>
                </div>

                <div class="wasm_frame">
                    <div class="wasm_title">{ "Exemple de code" }</div>
                    <div class="wasm_content">
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
