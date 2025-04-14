pub enum Message {
    InitWorms,
    SwitchScene(crate::scene::Scene), // sao <3
}

pub struct App {
    current_scene: crate::scene::Scene,
    canvas_node_ref: yew::NodeRef,
}

#[derive(Debug, PartialEq, yew::Properties)]
pub struct Props {
    pub scenes: Vec<crate::scene::Scene>,
    pub default_scene_index: usize,
}

impl yew::Component for App {
    type Message = Message;
    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let scenes = &ctx.props().scenes;
        let current_scene = *scenes
            .get(ctx.props().default_scene_index)
            .or_else(|| scenes.first())
            .unwrap();

        Self {
            current_scene,
            canvas_node_ref: yew::NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        use {
            crate::component::start_wormgrid,
            wasm_bindgen::JsCast as _,
            web_sys::{window, HtmlCanvasElement, WebGlRenderingContext},
        };

        match msg {
            Message::InitWorms => {
                let canvas = self.canvas_node_ref.cast::<HtmlCanvasElement>().unwrap();
                let w = window().unwrap();
                canvas.set_width(w.inner_width().unwrap().as_f64().unwrap() as u32);
                canvas.set_height(w.inner_height().unwrap().as_f64().unwrap() as u32);

                let glctx = canvas
                    .get_context("webgl")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<WebGlRenderingContext>()
                    .unwrap();

                start_wormgrid(glctx);
                true
            }
            Message::SwitchScene(scene) => {
                self.current_scene = scene;
                true
            }
        }
    }

    // fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
    //     use {crate::scene::Scene, gloo::console::log, js_sys::Date, yew::html};

    //     log!("Draw");

    //     let scenes = ctx.props().scenes.clone();

    //     html! {
    //         <div id="global">
    //         <div id="header">
    //             <a class="header_item" href="http://github.com/Bowarc/wasm_portfolio">
    //                 <img src="resources/github.webp" alt="Github icon" class="icon"/>
    //             </a>
    //             <div id="scene_list" class="header_item">{
    //                 scenes.into_iter().map(|scene|{
    //                     let current = if self.current_scene == scene{
    //                         "current"
    //                     }else{
    //                         ""
    //                     };
    //                     html!{
    //                         <button class={format!("scene_button {current}")} onclick={ctx.link().callback(|_| Message::SwitchScene(scene))}>
    //                             { format!("{scene}") }
    //                         </button>
    //                     }
    //                 }).collect::<Vec<yew::virtual_dom::VNode>>()
    //             }</div>
    //         </div>
    //         <div id="content">
    //             <canvas id="gridworm_canvas" ref={self.canvas_node_ref.clone()} />
    //             {
    //                 self.current_scene.html()
    //             }
    //         </div>
    //         <footer>
    //             { format!("Rendered: {}", String::from(Date::new_0().to_string()))}
    //         </footer>
    //         </div>
    //     }
    // }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        use {crate::component::NotificationManager, js_sys::Date, yew::html};

        let scenes = ctx.props().scenes.clone();

        html! {
            <div id="global">
            <div id="header">
                <a class="header_item" href="http://github.com/Bowarc/leaguecord">
                    <img src="/resources/github.webp" alt="Github icon" class="icon"/>
                </a>
                <div id="scene_list" class="header_item">{
                    scenes.into_iter().map(|scene|{
                        html!{
                            <button class={format!("scene_button{}", if self.current_scene == scene {" current"} else {""})} onclick={ctx.link().callback(move |_| Message::SwitchScene(scene))}>
                                { format!("{scene}") }
                            </button>
                        }
                    }).collect::<Vec<yew::virtual_dom::VNode>>()
                }</div>
            </div>
            <div id="content">
                <canvas id="gridworm_canvas" ref={self.canvas_node_ref.clone()} />
                {
                    self.current_scene.html(ctx)
                }
                <NotificationManager />
            </div>
            <footer>
                { format!("Rendered: {}", String::from(Date::new_0().to_string())) }
            </footer>
            </div>
        }
    }
    fn rendered(&mut self, ctx: &yew::Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Message::InitWorms);
        }
    }
}
