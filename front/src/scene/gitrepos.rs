use crate::component::GitProjectList;
use yew::{function_component, html, Html};
use i18nrs::yew::use_translation;

#[function_component]
pub fn GitRepos() -> Html {
    let (i18n, _) = use_translation();
    
    html! {<>
        <h3>
        { i18n.t("gitrepos.title") }
        </h3>
        <br/>
        <GitProjectList />
    </>}
}
