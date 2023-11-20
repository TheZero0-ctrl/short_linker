use crate::prelude::*;
use reqwest::Client;

pub struct ShrinkmeApi {
    client: Client,
    base_url: String
}

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    pub status: String,
    #[serde(rename = "shortenedUrl")]
    pub shortened_url: String
}

impl ShrinkmeApi {
    pub fn new() -> Self {
        let base_url = String::from("https://shrinkme.io/api");
        Self {
            client: reqwest::Client::new(),
            base_url
        }
    }

    pub async fn shrink_link(&self, link: String) -> Result<APIResponse, Error>{
        println!("{}", "Shrinking Link ...".yellow());
        let token = env::var("SHRINKME_TOKEN").unwrap();

        let response = self.client
            .get(self.base_url.clone() + "?api=" + &token + "&url=" + &link)
            .send()
            .await?
            .json::<APIResponse>()
            .await?;

        Ok(response)
    }
}
