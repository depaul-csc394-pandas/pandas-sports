use actix_web::{error::BlockingError, HttpResponse};
use failure::{Error, Fail};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Status {
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Conflict = 409,
    InternalServerError = 500,
    ServiceUnavailable = 503,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            *self as u32,
            match self {
                Status::Unauthorized => "Unauthorized",
                Status::Forbidden => "Forbidden",
                Status::NotFound => "Not Found",
                Status::Conflict => "Conflict",
                Status::InternalServerError => "Internal Server Error",
                Status::ServiceUnavailable => "Service Temporarily Unavailable",
            }
        )
    }
}

#[derive(Debug, Fail)]
pub struct ServiceError {
    status: Status,
    cause: Option<Error>,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cause_string = match self.cause {
            Some(ref c) => format!(": {}", c),
            None => "".to_string(),
        };

        write!(f, "Service error ({}){}", self.status, cause_string,)
    }
}

impl std::convert::Into<actix_web::Error> for ServiceError {
    fn into(self) -> actix_web::Error {
        use actix_web::error;

        match self.cause {
            Some(c) => match self.status {
                Status::Unauthorized => error::ErrorUnauthorized(c),
                Status::Forbidden => error::ErrorForbidden(c),
                Status::NotFound => error::ErrorNotFound(c),
                Status::Conflict => error::ErrorConflict(c),
                Status::InternalServerError => error::ErrorInternalServerError(c),
                Status::ServiceUnavailable => error::ErrorServiceUnavailable(c),
            },

            None => match self.status {
                Status::Unauthorized => HttpResponse::Unauthorized(),
                Status::Forbidden => HttpResponse::Forbidden(),
                Status::NotFound => HttpResponse::NotFound(),
                Status::Conflict => HttpResponse::Conflict(),
                Status::InternalServerError => HttpResponse::InternalServerError(),
                Status::ServiceUnavailable => HttpResponse::ServiceUnavailable(),
            }
            .finish()
            .into(),
        }
    }
}

#[inline(always)]
fn construct_error<E>(e: E, status: Status) -> ServiceError
where
    E: Into<Error>,
{
    ServiceError {
        status,
        cause: Some(e.into()),
    }
}

pub fn unauthorized<E>(e: E) -> ServiceError
where
    E: Into<Error>,
{
    construct_error(e, Status::Unauthorized)
}

#[allow(dead_code)]
pub fn forbidden<E>(e: E) -> ServiceError
where
    E: Into<Error>,
{
    construct_error(e, Status::Forbidden)
}

pub fn not_found<E>(e: E) -> ServiceError
where
    E: Into<Error>,
{
    construct_error(e, Status::NotFound)
}

pub fn conflict<E>(e: E) -> ServiceError
where
    E: Into<Error>,
{
    construct_error(e, Status::Conflict)
}

pub fn internal<E>(e: E) -> ServiceError
where
    E: Into<Error>,
{
    construct_error(e, Status::InternalServerError)
}

pub fn unavailable<E>(e: E) -> ServiceError
where
    E: Into<Error>,
{
    construct_error(e, Status::ServiceUnavailable)
}

pub fn from_blocking(be: BlockingError<ServiceError>) -> ServiceError {
    match be {
        BlockingError::Error(e) => e,
        BlockingError::Canceled => ServiceError {
            status: Status::InternalServerError,
            cause: None,
        },
    }
}
