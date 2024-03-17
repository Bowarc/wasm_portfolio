#[macro_use(trace, debug, info, warn, error)]
extern crate log;

mod response;
mod routes;

#[rocket::main]
async fn main() {
    let logcfg = logger::LoggerConfig::new()
        .set_level(log::LevelFilter::Trace)
        .add_filter("rocket", log::LevelFilter::Warn);
    logger::init(logcfg, Some("log/server.log"));

    // Small print to show the start of the program log in the file
    trace!(
        "\n╭{line}╮\n│{message:^30}│\n╰{line}╯",
        line = "─".repeat(30),
        message = "Program start"
    );

    let rocket = rocket::build()
        // .manage(cache)
        .register("/", rocket::catchers![])
        .register("/upload", rocket::catchers![])
        .mount(
            "/",
            rocket::routes![
                routes::root,
                routes::style,
                routes::wormscss,
                routes::gitcardcss,
                routes::front,
                routes::wasm,
                routes::github_icon,
                routes::rust_icon,
                routes::python_icon,
            ],
        )
        .ignite()
        .await
        .unwrap();

    display_config(rocket.config(), rocket.routes(), rocket.catchers());

    rocket.launch().await.unwrap();
}

/// Displays the config in the console
fn display_config<'a>(
    rocket_cfg: &rocket::Config,
    rocket_routes: impl Iterator<Item = &'a rocket::Route>,
    rocket_catchers: impl Iterator<Item = &'a rocket::Catcher>,
) {
    let profile = rocket_cfg.profile.as_str().as_str();
    let address = rocket_cfg.address;
    let port = rocket_cfg.port;
    let workers = rocket_cfg.workers;
    // let max_blocking = cfg.max_blocking;
    let indent = rocket_cfg.ident.as_str().unwrap_or("[ERROR] Undefined");
    let ip_headers = rocket_cfg
        .ip_header
        .as_ref()
        .map(|header| header.as_str())
        .unwrap_or("[ERROR] Undefined");
    let limits = ["bytes", "data-form", "file", "json", "msgpack", "string"]
        .iter()
        .map(|limit_name| {
            format!(
                "{limit_name}: {}",
                rocket_cfg
                    .limits
                    .get(limit_name)
                    .unwrap_or(rocket::data::ByteUnit::from(0))
            )
        })
        .collect::<Vec<String>>();
    let keep_alive_s = rocket_cfg.keep_alive;
    let shutdown_mode = &rocket_cfg.shutdown;

    let routes = rocket_routes
        .map(|route| {
            let uri = route.uri.origin.to_string();
            let name = route
                .name
                .as_ref()
                .map(|name| name.as_ref())
                .unwrap_or("[ERROR] Undefined");
            let method = route.method.as_str();
            format!("{method:<7} {uri:<15} {name}")
        })
        .collect::<Vec<String>>();

    let catchers = rocket_catchers
        .map(|catcher| {
            let base = catcher.base.to_string();
            let name = catcher
                .name
                .as_ref()
                .map(|name| name.as_ref())
                .unwrap_or("[ERROR] Undefined");
            let code = catcher
                .code
                .map(|code| code.to_string())
                .unwrap_or("[ERROR] Undefined".to_string());

            format!("{code:<7} {base:<15} {name}")
        })
        .collect::<Vec<String>>();

    let display_vec = |data: Vec<String>| -> String {
        let mut out = String::new();
        out.push_str("[\n");
        for d in data {
            out.push_str(&format!(" {d}\n"))
        }
        out.push(']');
        out
    };

    info!("Config:\nUsing profile: {profile}\nAddress: {address}:{port}\nWorkers: {workers}\nIndent: {indent}\nHeaders: {ip_headers}\nLimits: {formatted_limits}\nConnection lifetime: {keep_alive_s}s\nShutdown mode: {shutdown_mode}\nRoutes: {formatted_routes}\nCatchers: {formatted_catchers}",
        formatted_limits = display_vec(limits),
        formatted_routes = display_vec(routes),
        formatted_catchers = display_vec(catchers)
    );
}
