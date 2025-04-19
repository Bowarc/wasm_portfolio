use gloo::{console::log, utils::format::JsValueSerdeExt as _};
use js_sys::Date;
use wasm_bindgen::JsCast as _;

// Order matters here, the first one will be the main one
const ACCOUNTS: [&str; 2] = ["Bowarc", "HugoLz"];

pub struct GitProjectList {
    repos: Vec<Repository>,
}

pub enum Msg {
    FetchRepos,
    FetchReposResult(crate::utils::FetchState<Vec<Repository>>),
}

// #[derive(serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Repository {
    name: String,
    owner_name: String,
    description: String,
    // created_date: Date,
    last_update: Date,
    language: String,
    // public: bool,
    // fork: bool,
    // size: i32,
}

impl yew::Component for GitProjectList {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        log!("GitProjectList comp created");
        ctx.link().send_message(Msg::FetchRepos);
        Self { repos: Vec::new() }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchRepos => {
                // log!("Fetch repos");
                ctx.link()
                    .send_message(Msg::FetchReposResult(crate::utils::FetchState::Fetching));

                ACCOUNTS.iter().for_each(|user| {
                    ctx.link().send_future(async {
                        let res = match fetch_repos(user).await {
                            Ok(repos) => {
                                // log!("Repos fetch success");
                                crate::utils::FetchState::Success(repos)
                            }
                            Err(err) => {
                                // log!("Repos fatch failled with error: {err}");
                                crate::utils::FetchState::Failed(err)
                            }
                        };
                        Msg::FetchReposResult(res)
                    });
                });
            }
            Msg::FetchReposResult(state) => {
                match state {
                    crate::utils::FetchState::Success(mut repos) => {
                        // log!(format!(
                        //     "Received repo fetch state: Success with {} repos",
                        //     repos.len()
                        // ));
                        self.repos.append(&mut repos);

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
                        log!(format!("Repo fetch failled with Error({why:?})"))
                    }
                }
            }
        }

        true
    }

    fn view(&self, _ctx: &yew::prelude::Context<Self>) -> yew::Html {
        use yew::html;

        let mut repos_to_display = self.repos.clone();

        let mut i = 0;
        while i < repos_to_display.len() {
            let repo = repos_to_display.get(i).unwrap();

            // gloo::console::log!(format!("{}", repo.name));

            if repo.description == "null" {
                // log!(format!("Removed due to no desc"));
                repos_to_display.remove(i);
                continue;
            }

            if [".cfg", ".nvim"].iter().any(|s| repo.name.contains(s)){
                // log!(format!("Removed because it's a config repo"));
                repos_to_display.remove(i);
                continue;
            }

            let Some(dup_pos) = repos_to_display
                .iter()
                .position(|r| r.name == repo.name && r.owner_name != repo.owner_name)
            else {
                i += 1;
                continue;
            };

            let dup_repo = repos_to_display.get(dup_pos).unwrap();

            // gloo::console::log!(format!(
            //     "duplicates\nfirst: {} by {} with date: {}\nseccond: {} by {} with date: {}",
            //     repo.name,
            //     repo.owner_name,
            //     repo.last_update.get_time(),
            //     dup_repo.name,
            //     dup_repo.owner_name,
            //     dup_repo.last_update.get_time()
            // ));

            let repo_last_update = repo.last_update.get_time();
            let dup_repo_last_update = dup_repo.last_update.get_time();

            let delete_idx = if repo_last_update == dup_repo_last_update {
                if &repo.owner_name == ACCOUNTS.first().unwrap() {
                    // log!(format!("Removing repo from {}", dup_repo.owner_name));
                    dup_pos
                } else {
                    // log!(format!("Removing repo from {}", repo.owner_name));
                    i
                }
            } else if repo_last_update > dup_repo_last_update {
                dup_pos
            } else {
                i
            };

            let _deleted = self.repos.get(delete_idx).unwrap();
            // log!(format!("Removing repo from: {}", deleted.owner_name));
            repos_to_display.remove(i);
        }

        html! {
            <div id="project_list">
                {repos_to_display.iter().map(|repo|{ html!{
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
                                    alt={ format!("{} ", repo.language) }
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
                                        repo.last_update.to_locale_date_string("fr-FR", &wasm_bindgen::JsValue::from_str("")),
                                        repo.last_update.get_hours(), repo.last_update.get_minutes()
                                    )
                                }
                                </span>
                            </div>
                        </a>
                    </div>
                }}).collect::<Vec<yew::Html>>()}
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &yew::prelude::Context<Self>, _first_render: bool) {
        log!("Repo rendered")
    }
}

async fn fetch_repos(user: &'static str) -> Result<Vec<Repository>, wasm_bindgen::JsValue> {
    use web_sys::Request;

    let opts = web_sys::RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(web_sys::RequestMode::Cors);

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

    let repos = json_data
        .into_serde::<Vec<serde_json::Value>>()
        .unwrap()
        .iter()
        .flat_map(|value| {
            let as_rs_string =
                |s: &serde_json::Value| -> String { s.to_string().replace(['"'], "") };
            let date_as_rs_string =
                |s: &serde_json::Value| -> String { as_rs_string(s).replace('Z', "") };

            Some(Repository {
                name: as_rs_string(value.get("name")?),
                owner_name: as_rs_string(value.get("owner")?.get("login")?),
                description: as_rs_string(value.get("description")?),
                // created_date: Date::new(&wasm_bindgen::JsValue::from_str(&date_as_rs_string(
                //     value.get("created_at")?,
                // ))),
                last_update: Date::new(&wasm_bindgen::JsValue::from_str(&date_as_rs_string(
                    value.get("pushed_at")?,
                ))),
                language: as_rs_string(value.get("language")?),
                // public: !value.get("private")?.as_bool()?,
                // fork: value.get("fork")?.as_bool()?,
                // size: value.get("size")?.as_i64()? as i32,
            })
        })
        .collect::<Vec<Repository>>();

    Ok(repos)
}
