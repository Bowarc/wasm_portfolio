#[path = "routes/bots.rs"]
mod bot_routes;
pub use bot_routes::{bot_admin, bot_env, bot_wordpress, bot_wp, bot_wp_admin};

#[rocket::get("/")]
pub async fn root(ip_addr: rocket_client_addr::ClientAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("index.html", ContentType::HTML, ip_addr, false).await
}

#[rocket::get("/front.js")]
pub async fn front_js(ip_addr: rocket_client_addr::ClientAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("/front.js", ContentType::JavaScript, ip_addr, true).await
}

#[rocket::get("/front_bg.wasm")]
pub async fn front_bg_wasm(ip_addr: rocket_client_addr::ClientAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("/front_bg.wasm", ContentType::WASM, ip_addr, true).await
}

#[rocket::get("/index.html")]
pub async fn index_html(ip_addr: rocket_client_addr::ClientAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("/index.html", ContentType::HTML, ip_addr, false).await
}

#[rocket::get("/favicon.ico")]
pub async fn favicon_ico(ip_addr: rocket_client_addr::ClientAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("favicon.ico", ContentType::Icon, ip_addr, true).await
}

#[rocket::get("/sitemap.xml")]
pub async fn sitemap_xml(ip_addr: rocket_client_addr::ClientAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("sitemap.xml", ContentType::Icon, ip_addr, true).await
}

#[rocket::get("/robots.txt")]
pub async fn robots_txt(ip_addr: rocket_client_addr::ClientAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("robots.txt", ContentType::Icon, ip_addr, true).await
}

// The goal of this method, is to not use FileServer (because i wanna make sure of what file i serve)
macro_rules! static_dir_server {
    ($path:literal, $dir:literal, $func_name:ident, $allowed_files:expr) => {
        #[rocket::get($path)]
        pub async fn $func_name(
            file: &str,
            ip_addr: rocket_client_addr::ClientAddr,
        ) -> super::response::Response {
            use super::response::Response;
            use rocket::http::Status;

            const ALLOWED_FILES: &[&str] = $allowed_files;

            if !ALLOWED_FILES.contains(&file) {
                return Response::builder().with_status(Status::NotFound).build();
            }

            serve_static(concat!("/", $dir), file, ip_addr, true).await
        }
    };
}

static_dir_server!(
    "/css/<file>",
    "css",
    static_css,
    &[
        "contact.css",
        "gitcard.css",
        "header.css",
        "hidable.css",
        "home.css",
        "light_switch.css",
        "locale_switch.css",
        "presentation.css",
        "style.css",
        "theme.css",
        "void.css",
        "worms.css",
    ]
);
static_dir_server!(
    "/resources/<file>",
    "resources",
    static_resource,
    &[
        "bash.webp",
        "c.webp",
        "cpp.webp",
        "csharp.webp",
        "css.webp",
        "docker.webp",
        "flag_en.webp",
        "flag_fr.webp",
        "git.webp",
        "github.webp",
        "html.webp",
        "java.webp",
        "javascript.webp",
        "kotlin.webp",
        "php.webp",
        "pwsh.webp",
        "pwsh2.webp",
        "python.webp",
        "rust.webp",
        "sql.jpg",
        "sql.webp",
        "ssh.webp",
        "storage_server.drawio.png",
        "storage_server.drawio100px.png",
        "storage_server.drawio200px.png",
        "zig.webp",
    ]
);
static_dir_server!(
    "/lib/hidable/<file>",
    "lib/hidable",
    static_js,
    &["hidable.js"]
);

pub async fn serve_static(
    path: &str,
    file: &str,
    ip_addr: rocket_client_addr::ClientAddr,
    cache: bool,
) -> super::response::Response {
    use rocket::http::ContentType;

    #[inline]
    fn ext(file_name: &str) -> Option<&str> {
        if !file_name.contains(".") {
            return None;
        }

        let dot_index = file_name.rfind(".").unwrap();

        Some(&file_name[(dot_index + 1)..file_name.len()])
    }

    let content_type = ext(file)
        .and_then(ContentType::from_extension)
        .unwrap_or_else(|| {
            error!("Could not infer content type of file: {file}, requested in {path}");
            ContentType::Any
        });

    static_file_response(&format!("{path}/{file}"), content_type, ip_addr, cache).await
}

async fn static_file_response(
    path: &str,
    content_type: rocket::http::ContentType,
    ip_addr: rocket_client_addr::ClientAddr,
    cache: bool,
) -> super::response::Response {
    use super::response::Response;
    use rocket::http::Status;
    use tokio::fs::File;

    match File::open(format!("./static/{path}")).await {
        Ok(file) => {
            // trace!("Static file query from {ip_addr}: {path}");
            let mut response = Response::builder()
                .with_status(Status::Ok)
                .with_content(file)
                .with_content_type(content_type);

            if cache {
                response = response.with_header("Cache-Control", "max-age=3600") // Ask the browser to cache the request for 1 hour, might help for server load
            }

            response.build()
        }
        Err(e) => {
            warn!("Static file query from {ip_addr}: {path} failed due to: {e}");
            Response::builder().with_status(Status::NotFound).build()
        }
    }
}
