mod gitrepos;
pub use gitrepos::GitRepos;
mod contatct;
pub use contatct::Contact;
// mod wasm;
// pub use wasm::WASM;
mod home;
pub use home::Home;
mod projects;
pub use projects::Projects;
mod void;
pub use void::Void;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Scene {
    Home,
    GitRepos,
    // WASMShowcase,
    Projects,
    Contact,
    Void,
}

impl Scene{
    pub fn html(&self, _ctx: &yew::Context<crate::app::App>) -> yew::virtual_dom::VNode{
        use yew::html;


        match self{
            Scene::Home => html!{<Home />},
            Scene::GitRepos => html!{<GitRepos />},
            // Scene::WASMShowcase => html!{<WASM />},
            Scene::Projects => html!{<Projects />},
            Scene::Contact => html!{<Contact />},
            Scene::Void => html!{<Void />}
        }
    }
}

impl std::fmt::Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scene::Home => write!(f, "Home"),
            Scene::GitRepos => write!(f, "Git repos"),
            // Scene::WASMShowcase => write!(f, "Web assembly"),
            Scene::Projects => write!(f, "Projects"),
            Scene::Contact => write!(f, "Contact"),
            Scene::Void => write!(f, "Void"),
        }
    }
}
