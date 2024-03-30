use yew::html;

pub struct Age;

impl yew::Component for Age {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::prelude::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        use js_sys::Date;
        use wasm_bindgen::JsValue;

        let birth_date = Date::new(&JsValue::from_f64(
            Date::new_0().get_time() - Date::new(&JsValue::from_str("2001, 9, 15")).get_time(),
        ));

        html! {<>
            { (birth_date.get_time()/ 1000. / 60. / 60. / 24. / 365.242199) as i32 }
            { " ans" }
        </>}
    }
}
