use gloo::console::log;
use yew::html;

pub struct WASM;

impl yew::Component for WASM {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        log!("update wasm");

        let document = web_sys::window().unwrap().document().unwrap();

        let create_script = |path: &str, id: &str| {
            let script = document.create_element("script").unwrap();
            script.set_attribute("src", path).unwrap();
            script.set_attribute("defer", "").unwrap();
            script.set_id(id);
            document.body().unwrap().append_child(&script).unwrap();
        };

        create_script("/prism.min.js","prism");
        create_script("/prism-autoloader.min.js","prism-autoloader");

        false
    }

    fn rendered(&mut self, ctx: &yew::prelude::Context<Self>, first_render: bool) {
        ctx.link().send_future(async {
            let document = web_sys::window().unwrap().document().unwrap();

            let remove_script = |id: &str|{
                if let Some(script_element) = document.get_element_by_id(id) {
                    let parent_node = script_element.parent_node().unwrap();
                    parent_node.remove_child(&script_element).unwrap();
                }
            };

            remove_script("prism");
            remove_script("prism-autoloader");

            gloo_timers::future::TimeoutFuture::new(25).await;
        });
    }
    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        log!("draw wasm");

        html! {<>
            <div id="wasm_view">
                <h1>{"Introduction à WebAssembly (Wasm)"}</h1>
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
                    <code class="language-rust">
                        {r#"
use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    value: i64, // This will store the counter value
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
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
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
                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>

                    // A button to send two Increment messages
                    <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
                        { "+1, +1" }
                    </button>

                </div>

                // Display the current value of the counter
                <p class="counter">
                    { self.value }
                </p>

                // Display the current date and time the page was rendered
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
                        "#}
                    </code>
                </pre>
                <p>{"Cet exemple affiche un simple compteur avec 3 boutons +1, -1 et +2 sur la page"}</p>
            </div>
        </>}
    }
}
