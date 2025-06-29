mod gitrepos;
mod not_found;
pub use gitrepos::GitRepos;
pub use not_found::NotFound;
mod contatct;
pub use contatct::Contact;
// mod wasm;
// pub use wasm::WASM;
mod home;
pub use home::Home;
// mod projects;
// pub use projects::Projects;
mod void;
pub use void::Void;
use yew::Callback;

use crate::Route;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Scene {
    Home,
    GitRepos,
    // WASMShowcase,
    // Projects,
    Contact,
    Void,
    NotFound,
}

impl Scene {
    pub fn html(&self, set_scene_cb: Callback<Scene>) -> yew::virtual_dom::VNode {
        use yew::html;

        match self {
            Scene::Home => html! {<Home {set_scene_cb}/>},
            Scene::GitRepos => html! {<GitRepos />},
            // Scene::WASMShowcase => html!{<WASM />},
            // Scene::Projects => html! {<Projects />},
            Scene::Contact => html! {<Contact />},
            Scene::Void => html! {<Void />},
            Scene::NotFound => html! {<NotFound />},
        }
    }
    pub fn route(&self) -> crate::Route {
        match self {
            Scene::Home => Route::Home,
            Scene::GitRepos => Route::Git,
            Scene::Contact => Route::Contact,
            Scene::NotFound => Route::NotFound,
            _ => Route::Default
        }
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scene::Home => write!(f, "Home"),
            Scene::GitRepos => write!(f, "Git repos"),
            // Scene::WASMShowcase => write!(f, "Web assembly"),
            // Scene::Projects => write!(f, "Projects"),
            Scene::Contact => write!(f, "Contact"),
            Scene::Void => write!(f, "Void"),
            Scene::NotFound => write!(f, "Not found"),
        }
    }
}
