use clap::Parser;
#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), about, long_about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    #[arg(required = true)]
    pub config_path: String,
}