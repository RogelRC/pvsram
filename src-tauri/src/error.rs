use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: String,
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError {
            message: err.to_string(),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
