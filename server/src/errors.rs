use std::fmt;

use ntex::{
    http::StatusCode,
    web::{HttpResponse, WebResponseError},
};

#[derive(Debug, Clone)]
pub enum CustomError {
    NotFound(String),
    InternalServerError(String),
}

impl WebResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self, _: &ntex::web::HttpRequest) -> HttpResponse {
        HttpResponse::new(self.status_code()).set_body(
            match self {
                Self::NotFound(e) => e,
                Self::InternalServerError(e) => e,
            }
            .into(),
        )
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::NotFound(e) => write!(f, "{e}"),
            CustomError::InternalServerError(e) => write!(f, "{e}"),
        }
    }
}
