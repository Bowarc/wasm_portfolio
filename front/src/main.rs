#[allow(unused_imports)]
#[macro_use(trace, debug, info, warn, error, log)]
extern crate gloo_console;

mod app;
mod component;
mod render;
mod scene;
mod utils;

#[derive(Debug, Clone, Copy, PartialEq, yew_router::Routable)]
pub enum Route {
    #[at("/")]
    Default,
    #[at("/home")]
    Home,
    #[at("/git")]
    Git,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[yew::function_component]
fn Router() -> yew::Html {
    use {
        scene::Scene,
        yew::html,
        yew_router::{BrowserRouter, Switch},
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={ |route: Route| {
                let (scenes, default_scene_index) = match route {
                    Route::Default | Route::Home => {
                        (vec![
                            Scene::Home,
                            Scene::GitRepos,
                            Scene::Contact,
                            Scene::Void
                        ],0)
                    }
                    Route::Git => {
                        (vec![
                            Scene::Home,
                            Scene::GitRepos,
                            Scene::Contact,
                            Scene::Void
                        ],1)
                    }
                    Route::Contact => {
                        (vec![
                            Scene::Home,
                            Scene::GitRepos,
                            Scene::Contact,
                            Scene::Void
                        ],2)
                    }
                    Route::NotFound => {
                        (vec![
                            Scene::NotFound
                        ],0)
                    },
                };
                html! { <app::App {scenes} {default_scene_index} /> }
            }} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Router>::new().render();
}
