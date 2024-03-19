use gloo::{console::log, utils::format::JsValueSerdeExt as _};
use js_sys::Date;
use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::Request;
use yew::{html, Html};

pub struct GitProjectList {
    repos: crate::utils::FetchState<Vec<Repository>>,
}

pub enum Msg {
    FetchRepos,
    FetchReposResult(crate::utils::FetchState<Vec<Repository>>),
}

// #[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct Repository {
    name: String,
    owner_name: String,
    description: String,
    created_date: Date,
    last_push_date: Date,
    language: String,
    public: bool,
    fork: bool,
    size: i32,
}

impl yew::Component for GitProjectList {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchRepos);
        Self {
            repos: crate::utils::FetchState::NotFetching,
        }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        match &self.repos{
            crate::utils::FetchState::NotFetching => {
                html!{<></>}
            },
            crate::utils::FetchState::Fetching => html!{<></>},
            crate::utils::FetchState::Success(repos) => {
                let cards = repos.iter().map(|repo|{
                    html!{
                        <div class="project card">
                            <a href={ format!("https://github.com/{}/{}", repo.owner_name, repo.name) }
                                    class="card_link">
                                <div class="card_bg"></div>
                                <div class="card_title">
                                    <img src={ 
                                            format!("./resources/{}.webp", 
                                                repo.language.to_lowercase()
                                            )
                                        }
                                        alt={ format!("{} icon", repo.language) }
                                        class="icon"
                                    />
                                    { &repo.name }
                                </div>
                                <div class="card_description">
                                    { &repo.description }
                                </div>
                                <div class="card_date_box">
                                    { "Last update: " }
                                    <span class="card_date">
                                    {
                                        format!(" {:?} {}h{}",
                                            repo.last_push_date.to_locale_date_string("fr-FR", &JsValue::from_str("")),
                                            repo.last_push_date.get_hours(), repo.last_push_date.get_minutes()
                                        )
                                    }
                                    </span>
                                </div>
                            </a>
                        </div> 
                    }
                }).collect::<Vec<Html>>();
                html! {
                    <div id="project_list">
                        { "Repositories" }
                        { cards }
                    </div>
                }
            },
            crate::utils::FetchState::Failed(_) => html!{<></>},
        }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchRepos => {
                log!("Fetch repos");
                ctx.link()
                    .send_message(Msg::FetchReposResult(crate::utils::FetchState::Fetching));
                ctx.link().send_future(async {
                    match fetch_repos("Bowarc").await {
                        Ok(repos) => {
                            log!("Repos fetch success");
                            Msg::FetchReposResult(crate::utils::FetchState::Success(repos))
                        }
                        Err(err) => {
                            log!("Repos fatch failled with error: {err}");
                            Msg::FetchReposResult(crate::utils::FetchState::Failed(err))
                        },
                    }
                });

            }
            Msg::FetchReposResult(repos) => {
                log!("Received repos fetch result");

                self.repos = repos
            },
        }

        true
    }

    fn changed(
        &mut self,
        ctx: &yew::prelude::Context<Self>,
        _old_props: &Self::Properties,
    ) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &yew::prelude::Context<Self>, first_render: bool) {
        log!("Repo rendered")
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, ctx: &yew::prelude::Context<Self>) {}
}

async fn fetch_repos(usr: &'static str) -> Result<Vec<Repository>, wasm_bindgen::JsValue> {
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::Cors);

    log!(opts.clone());

    let request = Request::new_with_str_and_init(
        &format!("https://api.github.com/users/{usr}/repos"),
        &opts,
    )?;

    log!(request.url());
    let window = gloo::utils::window();
    let resp_value =
        wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();

    let json_data = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
    log!(json_data.clone());

    let repos = json_data
        .into_serde::<Vec<serde_json::Value>>()
        .unwrap()
        .iter()
        .flat_map(|value| {
            log!(format!("name: {:?}", value.get("name")));

            let sanitize_string = |s: String| -> String{ s.replace(&['"', ' ',],"")}; 

            Some(Repository {
                name: sanitize_string(value.get("name")?.to_string()),
                owner_name:  sanitize_string(value.get("owner")?.get("login")?.to_string()),
                description:  sanitize_string(value.get("description")?.to_string()),
                created_date: Date::new(&JsValue::from_str(&value.get("created_at")?.to_string())),
                last_push_date: Date::new(&JsValue::from_str(&value.get("updated_at")?.to_string())),
                language:  sanitize_string(value.get("language")?.to_string()),
                public: !value.get("private")?.as_bool()?,
                fork: value.get("fork")?.as_bool()?,
                size: value.get("size")?.as_i64()? as i32,
            })
        })
        .collect::<Vec<Repository>>();

    Ok(repos)
}
