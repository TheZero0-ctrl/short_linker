use crate::{prelude::*, zipper};
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use std::path::Path;


pub struct UpFilesApi {
    client: Client,
    base_url: String,
}

#[derive(Deserialize, Debug)]
pub struct APIResponse {
    pub status: String,
    pub url: String
}

impl UpFilesApi {
    pub fn new() -> Self {
        let base_url = String::from("https://api.upfiles.com/upload");
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }    

    pub async fn upload_file(&self, file_path: String) -> Result<(), Error> {
        match zipper::zip_folder(&file_path) {
            Ok(_) => {
                println!("{}", "Uploading to Upfiles ...".yellow());
                let token = env::var("UPFILES_TOKEN").unwrap();

                let path = Path::new(&file_path);
                let file_name = path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                let mut ancestors = path.ancestors();
                ancestors.next();
                let dist_dir = ancestors
                    .next()
                    .unwrap()
                    .join(file_name.clone() + ".zip");

                let file_content = std::fs::read(&dist_dir)
                    .expect("Failed to read file");
                let form = Form::new()
                    .part("token", Part::text(token))
                    .part("file", Part::bytes(file_content)
                    .file_name(file_name));

                let response = self.client
                    .post(&self.base_url)
                    .multipart(form)
                    .send()
                    .await?
                    .json::<APIResponse>()
                    .await?;
                let status = response.status;
                if status == "success" {
                    let url = response.url;
                    println!("{url}")
                } else {
                    println!("{}", "could not upload file".red())
                }
            }
            Err(e) => println!("Error: {e:?}"),
        }
        Ok(())
    }
}
