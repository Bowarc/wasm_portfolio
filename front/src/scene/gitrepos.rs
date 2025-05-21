use crate::component::GitProjectList;
use yew::{function_component, html, Html};
use i18nrs::yew::use_translation;

#[function_component]
pub fn GitRepos() -> Html {
    if let Some(nav) = yew_router::hooks::use_navigator() {
        nav.replace(&crate::Route::Git)
    }else{
        error!("Failed to retrieve the navigator")
    }

    let (i18n, _) = use_translation();
    
    html! {<div id="gitrepos">
        <h3>
        { i18n.t("gitrepos.title") }
        </h3>
        <br/>
        <GitProjectList />
    </div>}
}
