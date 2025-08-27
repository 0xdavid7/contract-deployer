use clap::Parser;

#[derive(Parser)]
#[command(
    name = "contract-deployer",
    version = "0.0.9",
    about = "Deploy smart contracts using TOML configuration",
    long_about = "A Rust-based binary that allows you to clone and deploy smart contracts using TOML configuration files, making deployments consistent and reproducible."
)]
pub struct Args {
    /// Configuration file path
    #[arg(short, long, value_name = "CONFIG_FILE")]
    pub config: String,

    /// Extra arguments for forge script
    #[arg(short, long, value_name = "ARGS")]
    pub args: Option<Vec<String>>,
}
