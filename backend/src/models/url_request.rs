use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UrlRequest {
    pub url: String,
}
