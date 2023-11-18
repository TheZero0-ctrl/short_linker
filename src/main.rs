use dotenv::dotenv;
mod cli;
mod commands;
mod upfiles;
mod zipper;

mod prelude {
    pub use crate::commands::*;
    pub use std::env;
    pub use crate::cli::*;
    pub use clap::{Parser, Subcommand, Args};
    pub use async_trait::async_trait;
    pub use anyhow::Error;
    pub use crate::upfiles::*;
    pub use crate::zipper::*;
    pub use serde::{Deserialize, Serialize};
    pub use colored::*;
}

use prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    Cli::parse().run().await;
}
