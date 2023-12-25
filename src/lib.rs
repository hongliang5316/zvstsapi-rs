use reqwest;

pub mod account;
pub mod order;

pub static BASE_URL: &str = "http://127.0.0.1:12345";

pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }
}

pub fn check_status_code(status: reqwest::StatusCode) -> Result<()> {
    if status.is_success() {
        Ok(())
    } else {
        Err(format!("status code: {}", status).into())
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;
