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
}

pub struct App {
    node_ref: yew::NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: yew::NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InitWorms => {
                // ctx.link().send_message(Msg::UpdateWorms);
                let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Draw");
        html! {
            <div>
                <canvas id="gridworm_canvas" ref={self.node_ref.clone()} />
                <component::Header/>
                <p id="description">
                    { "Hellow.\nJe suis un développeur autodidacte de " }
                    <component::Age/>
                    { ", spécialisé dans le développement logiciel et backend. J'ai commencé mon parcours avec Python et aujourd'hui j'utilise principalement Rust." }
                </p>
                <component::GitProjectList />
                // Display the current date and time the page was rendered
                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>
                {
                    "salut"
                }
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

                if window_size != wormgrid.size(){
                    glctx.canvas().unwrap().dyn_into::<HtmlCanvasElement>().unwrap().set_width(window_size.x as u32);
                    glctx.canvas().unwrap().dyn_into::<HtmlCanvasElement>().unwrap().set_height(window_size.y as u32);
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
fn main() {
    yew::Renderer::<App>::new().render();
}
