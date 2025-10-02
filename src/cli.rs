use clap::Parser;

#[derive(Parser)]
#[command(
    name = "contract-deployer",
    version = env!("CARGO_PKG_VERSION"),
    about = "Deploy smart contracts using TOML configuration",
    long_about = "A Rust-based binary that allows you to clone and deploy smart contracts using TOML configuration files, making deployments consistent and reproducible."
)]
pub struct Args {
    /// Configuration file path
    #[arg(short, long, value_name = "CONFIG_FILE")]
    pub config: String,

    /// Skip confirmation prompt and auto-confirm deployment
    #[arg(
        short('y'),
        long,
        help = "Skip confirmation prompt and auto-confirm deployment"
    )]
    pub skip_confirmation: bool,
}
