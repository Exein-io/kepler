use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder};

#[derive(Debug)]
pub enum ApplicationError {
    InternalServerError,
    BadRequest(String),
    ServiceUnavailable,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl actix_web::error::ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        let mut b = HttpResponseBuilder::new(self.status_code());

        if let Self::BadRequest(err) = self {
            b.body(err.to_owned())
        } else {
            b.finish()
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::ServiceUnavailable => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
