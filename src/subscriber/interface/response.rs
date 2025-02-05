use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub struct Response {
    status_code: StatusCode,
    body: Option<ResponseBody>,
}

#[derive(serde::Serialize)]
struct ResponseBody {
    message: String,
}

impl Response {
    pub fn new(status_code: StatusCode, message: Option<String>) -> Self {
        let mut body = None;
        if let Some(message) = message {
            body = Some(ResponseBody { message });
        }
        Response { status_code, body }
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        if self.body.is_some() {
            (self.status_code, Json(self.body)).into_response()
        } else {
            self.status_code.into_response()
        }
    }
}
