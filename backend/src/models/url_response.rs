use serde::Serialize;

#[derive(Serialize)]
pub struct UrlResponse {
    pub short_url: String,
    pub original_url: String,
}

impl UrlResponse {
    pub fn new(base_url: &str, short_code: &str, original_url: &str) -> Self {
        Self {
            short_url: format!("{}/{}", base_url, short_code),
            original_url: original_url.to_string(),
        }
    }
}
