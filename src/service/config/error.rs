#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read var: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("Failed to parse int var: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

