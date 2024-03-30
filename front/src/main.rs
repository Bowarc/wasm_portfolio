use gloo::console::{self, log};
use js_sys::Date;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext};
use yew::{html, Component, Context, Html};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use wasm_bindgen::prelude::*;

mod component;
mod render;
mod utils;

// Define the possible messages which can be sent to the component
pub enum Msg {
    InitWorms,
    SwitchScene(Scene), // sao <3
}

#[derive(Clone, Copy, PartialEq)]
pub enum Scene {
    Main,
    GitRepos,
    BTSResume,
}

pub struct App {
    current_scene: Scene,
    canvas_node_ref: yew::NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current_scene: Scene::Main,
            canvas_node_ref: yew::NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InitWorms => {
                // ctx.link().send_message(Msg::UpdateWorms);
                let canvas = self.canvas_node_ref.cast::<HtmlCanvasElement>().unwrap();
                let w = window().unwrap();
                canvas.set_width(w.inner_width().unwrap().as_f64().unwrap() as u32);
                canvas.set_height(w.inner_height().unwrap().as_f64().unwrap() as u32);
                let glctx: WebGlRenderingContext = canvas
                    .get_context("webgl")
                    .unwrap()
                    .unwrap()
                    .dyn_into()
                    .unwrap();

                Self::start_wormgrid(glctx);
                true
            }
            Msg::SwitchScene(scene) => {
                self.current_scene = scene;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Draw");
        let scene_html = match self.current_scene {
            Scene::Main => {
                html! {<>
                    <p id="description">
                        { "Hellow.\nJe suis un développeur autodidacte de " }
                        <component::Age/>
                        { ", spécialisé dans le développement logiciel et backend. J'ai commencé mon parcours avec Python et aujourd'hui j'utilise principalement Rust." }
                    </p>
                </>}
            }
            Scene::GitRepos => {
                html! {<>
                    <component::GitProjectList />
                </>}
            }
            Scene::BTSResume => {
                html! {<>

                </>}
            }
        };

        html! {
            <div id="global">
            <div id="header">
                <a class="header_item" href="http://github.com/Bowarc">
                    <img src="resources/github.webp" alt="Github icon" class="icon"/>
                </a>
                <div id="scene_list" class="header_item">
                {
                    [ Scene::Main, Scene::GitRepos, Scene::BTSResume ].iter().map(|scene|{
                        let current = if &self.current_scene == scene{
                            "current"
                        }else{
                            ""
                        };
                        html!{
                            <button class={format!("scene {current}")} onclick={ctx.link().callback(|_| Msg::SwitchScene(*scene))}>
                                { format!("{scene}") }
                            </button>
                        }
                    }).collect::<Vec<yew::virtual_dom::VNode>>()
                }
                </div>
            </div>
            <div id="main">
                <canvas id="gridworm_canvas" ref={self.canvas_node_ref.clone()} />
                { scene_html }
            </div>
            <footer>
                // Display the current date and time the page was rendered
                { "Rendered: " }
                { String::from(Date::new_0().to_string()) }
            </footer>
            </div>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::InitWorms);
        }
    }
}

impl App {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn start_wormgrid(glctx: WebGlRenderingContext) {
        // This should log only once -- not once per frame

        render::init(&glctx);

        let canvas_size = maths::Point::new(
            glctx.drawing_buffer_width() as f64,
            glctx.drawing_buffer_height() as f64,
        );

        let blob = maths::Rect::new((0.1, 0.1), (10., 10.), 0.);

        log!(format!("Canvas size: {canvas_size}"));

        // Gloo-render's request_animation_frame has this extra closure
        // wrapping logic running every frame, unnecessary cost.
        // Here constructing the wrapped closure just once.

        let cb = std::rc::Rc::new(std::cell::RefCell::new(None));

        *cb.borrow_mut() = Some(wasm_bindgen::closure::Closure::wrap(Box::new({
            let rect_shader_program = render::setup_shader(&glctx, "rect");
            let circle_shader_program = render::setup_shader(&glctx, "circle");
            let glctx = glctx.clone();
            let cb = cb.clone();
            let mut wormgrid = component::WormGrid::new(canvas_size, 20);
            let color = render::Color::random_rgb();
            move || {
                glctx.clear(
                    WebGlRenderingContext::COLOR_BUFFER_BIT
                        | WebGlRenderingContext::DEPTH_BUFFER_BIT,
                );
                let window_size = maths::Point::new(
                    window().unwrap().inner_width().unwrap().as_f64().unwrap(),
                    window().unwrap().inner_height().unwrap().as_f64().unwrap(),
                );

                if window_size != wormgrid.size() {
                    glctx
                        .canvas()
                        .unwrap()
                        .dyn_into::<HtmlCanvasElement>()
                        .unwrap()
                        .set_width(window_size.x as u32);
                    glctx
                        .canvas()
                        .unwrap()
                        .dyn_into::<HtmlCanvasElement>()
                        .unwrap()
                        .set_height(window_size.y as u32);
                }

                // render::draw(
                //     &glctx,
                //     &rect_shader_program,
                //     &render::rect_to_vert(maths::Rect::new((0., 0.), canvas_size, 0.), canvas_size),
                //     color,
                // );
                wormgrid.update(window_size);
                wormgrid.draw(&glctx, &rect_shader_program, &circle_shader_program);

                crate::render::end_frame(cb.borrow().as_ref().unwrap())
            }
        })
            as Box<dyn FnMut()>));

        crate::render::end_frame(cb.borrow().as_ref().unwrap());
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scene::Main => {
                write!(f, "Main menu")
            }
            Scene::GitRepos => {
                write!(f, "Git repos")
            }
            Scene::BTSResume => {
                write!(f, "Void")
            }
        }
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
