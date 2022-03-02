use thiserror::Error;

/// UserSessionError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum UserSessionError {
    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),
}
