use std::sync::atomic::Ordering;
use yew::{function_component, html, use_force_update, Callback, Html};

#[function_component]
pub fn Void() -> Html {
    use crate::component::WORM_DEBUG_DRAW_VISION_POINTS;

    let force_update = use_force_update();
        
    let onclick = Callback::from(move |_| {
        use crate::component::WORM_DEBUG_DRAW_VISION_POINTS;
        WORM_DEBUG_DRAW_VISION_POINTS.fetch_not(Ordering::AcqRel);

        debug!("toggled debug vision");
        force_update.force_update();
    });

    html! {<>
        <div class="void">
            <button class={if WORM_DEBUG_DRAW_VISION_POINTS.load(Ordering::Acquire){"on"}else{"off"}}onclick={onclick}>{"Toggle debug vision"}</button>
        </div>
    </>}
}

// pub struct Void;

// pub enum Message {
//     ToggleWormDebugVision,
// }

// impl yew::Component for Void {
//     type Message = Message;

//     type Properties = ();

//     fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
//         Self
//     }

//     fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
//         use crate::component::WORM_DEBUG_DRAW_VISION_POINTS;
//         match msg {
//             Message::ToggleWormDebugVision => unsafe {
//                 WORM_DEBUG_DRAW_VISION_POINTS = !WORM_DEBUG_DRAW_VISION_POINTS;
//                 gloo::console::log!(format!("Debug: {WORM_DEBUG_DRAW_VISION_POINTS}"));
//                 true
//             },
//         }
//     }

//     fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
//         use crate::component::WORM_DEBUG_DRAW_VISION_POINTS;

//         let onclick = ctx.link().send_message(|_| {{

//         use crate::component::WORM_DEBUG_DRAW_VISION_POINTS;

//                unsafe{WORM_DEBUG_DRAW_VISION_POINTS = !WORM_DEBUG_DRAW_VISION_POINTS}
//                ()
//         }});

//         html! {<>
//             <div class="void">
//                 <button class={if unsafe{WORM_DEBUG_DRAW_VISION_POINTS}{"on"}else{"off"}}onclick={ctx.link().callback(|_|Message::ToggleWormDebugVision)}>{"Toggle debug vision"}</button>
//             </div>
//         </>}
//     }
// }
