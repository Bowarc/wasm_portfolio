use rocket::{
    http::Status,
    serde::json::serde_json::{json, Value as JsonValue},
};

pub struct JsonApiResponse {
    json: JsonValue,
    status: Status,
    headers: std::collections::HashMap<String, String>,
}

impl<'r> rocket::response::Responder<'r, 'static> for JsonApiResponse {
    fn respond_to(self, req: &rocket::Request) -> rocket::response::Result<'static> {
        let mut resp = rocket::Response::build_from(self.json.respond_to(req).unwrap());

        let mut resp = resp.status(self.status);

        for (name, value) in self.headers {
            resp = resp.raw_header(name, value);
        }

        let out = resp.ok();
        // trace!("{out:?}");

        out
    }
}

pub struct Response {
    pub status: Status,
    pub content: Vec<u8>,
    pub content_type: rocket::http::ContentType, // C TYPE badeu :D
}

impl<'r> rocket::response::Responder<'r, 'static> for Response {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build()
            .header(self.content_type)
            .status(self.status)
            .sized_body(self.content.len(), std::io::Cursor::new(self.content))
            .ok()
    }
}
