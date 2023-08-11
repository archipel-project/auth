use super::error;

#[derive(Clone, Debug)]
pub struct Env {
    pub port: u16,
    pub host: String,
}

impl Env {
    pub fn load() -> Result<Self, error::ConfigError> {
        let port = std::env::var("PORT").unwrap_or("25082".into()).parse()?;
        let host = std::env::var("HOST").unwrap_or("0.0.0.0".into());

        Ok(Self { port, host })
    }

    pub fn get_server(&self) -> String {
        return format!("{}:{}", self.host, self.port)
    }
}