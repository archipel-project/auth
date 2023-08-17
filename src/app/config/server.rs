use super::error;

#[derive(Clone, Debug)]
pub struct Server {
    pub port: u16,
    pub host: String,
    pub db_url: String,
}

impl Server {
    pub fn load() -> Result<Self, error::ConfigError> {
        let port = std::env::var("PORT").unwrap_or("25082".into()).parse()?;
        let host = std::env::var("HOST").unwrap_or("0.0.0.0".into());
        let db_url = std::env::var("DATABASE_URL").unwrap_or("0.0.0.0".into());

        Ok(Self { port, host, db_url })
    }

    pub fn formatted(&self) -> String {
        return format!("{}:{}", self.host, self.port)
    }
}