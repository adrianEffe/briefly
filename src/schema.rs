use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateShortUrlSchema {
    pub url: String,
}
