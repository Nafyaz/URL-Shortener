use serde::Serialize;

#[derive(Serialize)]
pub struct UrlResponse {
    pub short_url: String,
    pub original_url: String,
}

impl UrlResponse {
    pub fn new(short_code: &str, original_url: &str) -> Self {
        Self {
            short_url: format!("{}", short_code),
            original_url: original_url.to_string(),
        }
    }
}
