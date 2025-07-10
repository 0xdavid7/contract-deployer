mod cli;
mod config;
mod deployer;
mod environment;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use deployer::ContractDeployer;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut deployer = ContractDeployer::new(&args.config)?;

    let extra_args: Vec<String> = args.args.unwrap_or_default();

    deployer.deploy(&extra_args).await?;

    Ok(())
}
