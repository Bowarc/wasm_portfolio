use i18nrs::yew::use_translation;
use yew::{function_component, html, virtual_dom::VNode, Callback, Html};

#[function_component]
pub fn LocaleSwitch() -> Html {
    let (_i18n, switch_cb) = use_translation();
    html! {<div class="header-item" id="locale_switch"><ul>{
        ["en", "fr"].into_iter().map(|locale|{
            html!{<li>
                <button
                    onclick={
                        let cb = switch_cb.clone();
                        Callback::from(move |_| {
                            debug!(format!("Request locale switch to: {locale}"));
                            cb.emit(locale.to_string())
                        })
                    }
                >
                    <img src={format!("/resources/flag_{locale}.webp")} alt={format!("{locale} flag")}/>
                </button></li>
            }
        }).collect::<Vec<VNode>>()
    }</ul></div>}
}
