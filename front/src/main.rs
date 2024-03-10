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
mod maths;
mod render;

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
    InitWorms,
    UpdateWorms,
}

pub struct App {
    value: Arc<Mutex<f32>>, // This will store the counter value
    node_ref: yew::NodeRef,
    rect_shader_program: Option<web_sys::WebGlProgram>,
    glctx: Option<WebGlRenderingContext>,
    wormgrid: Option<component::WormGrid>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: Mutex::new(0.0).into(),
            node_ref: yew::NodeRef::default(),
            rect_shader_program: None,
            glctx: None,
            wormgrid: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                *self.value.lock().unwrap() += 1.0;
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                *self.value.lock().unwrap() -= 1.0;
                console::log!("minus one");
                true
            }
            Msg::InitWorms => {
                log!("Init");
                // ctx.link().send_message(Msg::UpdateWorms);
                let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
                let w = window().unwrap();
                canvas.set_width(w.inner_width().unwrap().as_f64().unwrap() as u32);
                canvas.set_height(w.inner_height().unwrap().as_f64().unwrap() as u32);
                log!("canvas");

                let glctx: WebGlRenderingContext = canvas
                    .get_context("webgl")
                    .unwrap()
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                log!("glctx");
                self.rect_shader_program = Some(render::setup(&glctx));
                log!("program");
                self.wormgrid = Some(component::WormGrid::new(
                    maths::Point::new(
                        glctx.drawing_buffer_width() as f64,
                        glctx.drawing_buffer_height() as f64,
                    ),
                    255,
                ));
                log!("wormgrid");
                self.glctx = Some(glctx);
                log!("Done");
                ctx.link().send_message(Msg::UpdateWorms);
                false
            }

            Msg::UpdateWorms => {
                let Some(wormgrid) = &mut self.wormgrid else {
                    return false;
                };

                let Some(glctx) = &self.glctx else {
                    return false;
                };

                let Some(rect_shader_prog) = &self.rect_shader_program else {
                    return false;
                };

                let canvas_size = maths::Point::new(
                    glctx.drawing_buffer_width() as f64,
                    glctx.drawing_buffer_height() as f64,
                );

                wormgrid.update(canvas_size);
                wormgrid.draw(&glctx, &rect_shader_prog);

                // ctx.link().send_message(Msg::UpdateWorms);

                // crate::render::end_frame(&wasm_bindgen::closure::Closure::wrap(Box::new({
                // move || {}
                // })));

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("Draw");
        html! {
            <div>
                <canvas id="gridWormCanvas" ref={self.node_ref.clone()} />
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
                    { *self.value.lock().unwrap() }
                </p>

                // Display the current date and time the page was rendered
                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>
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

        let mut timestamp = 0.0;

        let shader_program = render::setup(&glctx);
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
            let rect_shader_prog = shader_program.clone();
            let glctx = glctx.clone();
            let cb = cb.clone();
            let mut wormgrid = component::WormGrid::new(canvas_size, 10);
            let color = render::Color::random_rgb();
            move || {
                // render::draw(
                //     &glctx,
                //     &rect_shader_prog,
                //     &render::rect_to_vert(maths::Rect::new((0., 0.), canvas_size, 0.), canvas_size),
                //     color,
                // );
                wormgrid.update(canvas_size);
                wormgrid.draw(&glctx, &rect_shader_prog);

                // crate::render::end_frame(cb.borrow().as_ref().unwrap())
            }
        })
            as Box<dyn FnMut()>));

        crate::render::end_frame(cb.borrow().as_ref().unwrap());
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
