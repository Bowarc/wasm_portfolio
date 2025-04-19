#[allow(unused_imports)]
#[macro_use(trace, debug, info, warn, error, log)]
extern crate gloo_console;

mod app;
mod component;
mod render;
mod scene;
mod utils;

#[yew::function_component]
fn Router() -> yew::Html {
    use {scene::Scene, yew::html};

    let scenes = vec![Scene::Home, Scene::GitRepos, Scene::Contact, Scene::Void];
    let default_scene_index = 0;

    html! {
        <app::App {scenes} {default_scene_index}/>
    }
}

fn main() {
    yew::Renderer::<Router>::new().render();
}
