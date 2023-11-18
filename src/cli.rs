use anyhow::Error;
use crate::prelude::*;
use std::process::ExitCode;

#[async_trait]
pub trait RunCommand{
    async fn run(self) -> Result<(), Error>;
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands  {
    /// upload file to ufiles
    UploadFile(upload_file::UploadFile)
}

impl Cli {
    pub async fn run(self) -> ExitCode {
        let output = match self.command {
            Commands::UploadFile(upload_file) => upload_file.run().await,
        };

        match output {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("{}", e);
                ExitCode::FAILURE
            }
        }
    }
}

