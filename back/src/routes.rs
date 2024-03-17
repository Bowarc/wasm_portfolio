use crate::response::Response;
use rocket::http::{ContentType, Status};

#[rocket::get("/")]
pub async fn root(remote_addr: std::net::SocketAddr) -> Response {
    let _old_msg = "

        Hi, please take a look at the /examples directory to understand how to use this api
    ";

    file_response("index.html", ContentType::HTML, remote_addr)
}

#[rocket::get("/style.css")]
pub async fn style(remote_addr: std::net::SocketAddr) -> Response {
    file_response("style.css", ContentType::CSS, remote_addr)
}
#[rocket::get("/worms.css")]
pub async fn wormscss(remote_addr: std::net::SocketAddr) -> Response {
    file_response("worms.css", ContentType::CSS, remote_addr)
}
#[rocket::get("/gitcard.css")]
pub async fn gitcardcss(remote_addr: std::net::SocketAddr) -> Response {
    file_response("gitcard.css", ContentType::CSS, remote_addr)
}


#[rocket::get("/front.js")]
pub async fn front(remote_addr: std::net::SocketAddr) -> Response {
    file_response("front.js", ContentType::JavaScript, remote_addr)
}

#[rocket::get("/front_bg.wasm")]
pub fn wasm(remote_addr: std::net::SocketAddr) -> Response {
    file_response("front_bg.wasm", ContentType::WASM, remote_addr)
}

#[rocket::get("/resources/github.webp")]
pub fn github_icon(remote_addr: std::net::SocketAddr) -> Response {
    file_response("resources/github.webp", ContentType::WASM, remote_addr)
}

#[rocket::get("/resources/rust.webp")]
pub fn rust_icon(remote_addr: std::net::SocketAddr) -> Response {
    file_response("resources/rust.webp", ContentType::WASM, remote_addr)
}

#[rocket::get("/resources/python.webp")]
pub fn python_icon(remote_addr: std::net::SocketAddr) -> Response {
    file_response("resources/python.webp", ContentType::WASM, remote_addr)
}

fn file_response(
    file_name: &str,
    content_type: ContentType,
    remote_addr: std::net::SocketAddr,
) -> Response {
    match read_static(file_name, remote_addr) {
        Some(bytes) => Response {
            status: Status::Ok,
            content: bytes,
            content_type: content_type,
        },
        None => Response {
            status: Status::InternalServerError,
            content: Vec::new(),
            content_type: ContentType::Plain,
        },
    }
}

fn read_static(file_name: &str, remote_addr: std::net::SocketAddr) -> Option<Vec<u8>> {
    use std::io::Read as _;
    trace!("New static file query from {remote_addr}: {file_name}");
    let mut buffer = Vec::new();
    let _size = std::fs::File::open(format!("./static/{file_name}"))
        .ok()?
        .read_to_end(&mut buffer)
        .ok()?;
    Some(buffer)
}
