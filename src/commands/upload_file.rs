use crate::prelude::*;

#[derive(Debug, Args)]
pub struct UploadFile {
    /// File Path
    #[arg(short, long)]
    file_path: String
}

#[async_trait]
impl RunCommand for UploadFile {
  async fn run(self) -> Result<(), Error> {
    let api = UpFilesApi::new();
    let _ = api.upload_file(self.file_path).await;
    Ok(())
  }
}
