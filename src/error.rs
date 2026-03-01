use thiserror::Error;

#[derive(Error, Debug)]

pub enum AppError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}