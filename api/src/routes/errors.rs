use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;
use tracing::error;

use crate::user_session::errors::UserSessionError;

impl IntoResponse for UserSessionError {
    fn into_response(self) -> Response {
        error!("{}", &self);

        let (status, error_message) = (StatusCode::INTERNAL_SERVER_ERROR, self.to_string());

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
