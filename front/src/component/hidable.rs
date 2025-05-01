use wasm_bindgen::prelude::wasm_bindgen;
use yew::{html, Component, Context, Html};

#[wasm_bindgen()]
extern "C" {
    // /lib/hidable/hidable.js
    pub fn update_visibles();
}

pub struct Hidable;

impl Component for Hidable {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let closure = Closure::new(Box::new(move || {
        //     error!("Hi :3");
        //     update_visibles();
        // }) as Box<dyn FnMut()>);

        // w.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
        //     .unwrap();

        // w.set_onload(Some(closure.as_ref().unchecked_ref()));
        // let Some(d_e) = w.document().and_then(|d| d.document_element()) else {
        //     error!("Could not get document element");
        //     return;
        // };

        html! {}
    }
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            update_visibles();
        }
    }
}
