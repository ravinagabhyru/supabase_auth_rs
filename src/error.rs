use kinded::Kinded;
use thiserror::Error;

#[derive(Debug, Default, Clone, Copy, Error, Kinded)]
pub enum AuthError {
    #[error("not authorized")]
    NotAuthorized,
    #[error("invalid parameters")]
    InvalidParameters,
    #[error("http error")]
    Http,
    #[error("internal library error")]
    Internal,
    #[error("resource not found")]
    NotFound,
    #[error("general gotrue error")]
    #[default]
    GeneralError,
}
