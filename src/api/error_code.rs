use axum::{http::StatusCode, response::{IntoResponse, Response}};

use crate::service::application_error::{ApplicationError, ApplicationErrorType};

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        let http_status_code = match self.error_type {
            ApplicationErrorType::NotFound => StatusCode::NOT_FOUND,
            ApplicationErrorType::ValidationFailed => StatusCode::UNPROCESSABLE_ENTITY,
        };

        (http_status_code, self.message).into_response()
    }
}