mod cli;
mod config;
mod deployer;
mod environment;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use deployer::ContractDeployer;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut deployer = ContractDeployer::new(
        &args.config,
        args.skip_confirmation,
        args.network,
        args.script,
    )?;

    deployer.deploy()?;

    Ok(())
}
