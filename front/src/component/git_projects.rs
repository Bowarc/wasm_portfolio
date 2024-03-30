use gloo::{console::log, utils::format::JsValueSerdeExt as _};
use js_sys::Date;
use serde_json::Value;
use wasm_bindgen::{JsCast as _, JsValue};
use web_sys::Request;
use yew::{html, Html};

pub struct GitProjectList {
    repos: Vec<Repository>,
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
    last_update: Date,
    language: String,
    public: bool,
    fork: bool,
    size: i32,
}

impl yew::Component for GitProjectList {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        log!("GitProjectList comp created");
        ctx.link().send_message(Msg::FetchRepos);
        Self { repos: Vec::new() }
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {
            <div id="project_list">
                {self.repos.iter().map(|repo|{
                    html!{
                        <div class="card">
                            <a href={ format!("https://github.com/{}/{}", repo.owner_name, repo.name) }
                                    class="card_link">
                                <div class="card_bg"></div>
                                <div class="card_title">
                                    <img src={
                                            format!("./resources/{}.webp",
                                                repo.language.to_lowercase()
                                            )
                                        }
                                        alt={ format!("icon ") }
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
                                        format!(" {} {}h{}",
                                            repo.last_update.to_locale_date_string("fr-FR", &JsValue::from_str("")),
                                            repo.last_update.get_hours(), repo.last_update.get_minutes()
                                        )
                                    }
                                    </span>
                                </div>
                            </a>
                        </div>
                    }
                }).collect::<Vec<Html>>()}
            </div>
        }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchRepos => {
                // log!("Fetch repos");
                ctx.link()
                    .send_message(Msg::FetchReposResult(crate::utils::FetchState::Fetching));

                ["Bowarc", "HugoLz"].iter().for_each(|user| {
                    ctx.link().send_future(async {
                        {
                            match fetch_repos(user).await {
                                Ok(repos) => {
                                    // log!("Repos fetch success");
                                    Msg::FetchReposResult(crate::utils::FetchState::Success(repos))
                                }
                                Err(err) => {
                                    // log!("Repos fatch failled with error: {err}");
                                    Msg::FetchReposResult(crate::utils::FetchState::Failed(err))
                                }
                            }
                        }
                    });
                });
            }
            Msg::FetchReposResult(state) => match state {
                crate::utils::FetchState::Success(repos) => {
                    // log!(format!(
                    //     "Received repo fetch state: Success with {} repos",
                    //     repos.len()
                    // ));
                    self.repos.append(
                        &mut repos
                            .into_iter()
                            .filter(|repo| repo.description != "null")
                            .filter(|repo| {
                                ![".nvim", ".cfg"]
                                    .iter()
                                    .map(|pattern| repo.name.contains(pattern))
                                    .any(|r| r)
                            })
                            .filter(|repo| !repo.fork)
                            .collect::<Vec<Repository>>(),
                    );
                    self.repos
                        .sort_by_key(|repo| -repo.last_update.get_time() as i64);
                }
                crate::utils::FetchState::NotFetching => {
                    // log!("Received repo fetch state: NotFetching")
                }
                crate::utils::FetchState::Fetching => {
                    // log!("Received repo fetch state: Fetching")
                }
                crate::utils::FetchState::Failed(why) => {
                    // log!(format!("Received repo fetch state: Error({why:?})"))
                }
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

async fn fetch_repos(user: &'static str) -> Result<Vec<Repository>, wasm_bindgen::JsValue> {
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::Cors);

    // log!(opts.clone());

    let request = Request::new_with_str_and_init(
        &format!("https://api.github.com/users/{user}/repos?per_page=100"),
        &opts,
    )?;

    // log!(request.url());
    let window = gloo::utils::window();
    let resp_value =
        wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: web_sys::Response = resp_value.dyn_into().unwrap();

    let json_data = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
    // log!(json_data.clone());

    let mut repos = json_data
        .into_serde::<Vec<serde_json::Value>>()
        .unwrap()
        .iter()
        .flat_map(|value| {
            let as_rs_string = |s: &Value| -> String { s.to_string().replace(&['"'], "") };
            let date_as_rs_string = |s: &Value| -> String { as_rs_string(s).replace('Z', "") };

            Some(Repository {
                name: as_rs_string(value.get("name")?),
                owner_name: as_rs_string(value.get("owner")?.get("login")?),
                description: as_rs_string(value.get("description")?),
                created_date: Date::new(&JsValue::from_str(&date_as_rs_string(
                    value.get("created_at")?,
                ))),
                last_update: Date::new(&JsValue::from_str(&date_as_rs_string(
                    value.get("pushed_at")?,
                ))),
                language: as_rs_string(value.get("language")?),
                public: !value.get("private")?.as_bool()?,
                fork: value.get("fork")?.as_bool()?,
                size: value.get("size")?.as_i64()? as i32,
            })
        })
        .collect::<Vec<Repository>>();

    Ok(repos)
}
