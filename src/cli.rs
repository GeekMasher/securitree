use clap::{Parser, Subcommand};


pub const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub const BANNER: &str = r#" _____                      _ _____
/  ___|                    (_)_   _|
\ `--.  ___  ___ _   _ _ __ _  | |_ __ ___  ___
 `--. \/ _ \/ __| | | | '__| | | | '__/ _ \/ _ \
/\__/ /  __/ (__| |_| | |  | | | | | |  __/  __/
\____/ \___|\___|\__,_|_|  |_| \_/_|  \___|\___|"#;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Enable Debugging
    #[clap(long, default_value_t = false)]
    pub debug: bool,

    /// Disable Banner
    #[clap(long, default_value_t = false)]
    pub disable_banner: bool,

    /// Configuration file path
    #[clap(
        short,
        long,
        default_value_t=String::from("./config.toml")
    )]
    pub config: String,

    #[clap(subcommand)]
    pub commands: ArgumentCommands,
}

#[derive(Subcommand, Debug)]
pub enum ArgumentCommands {
    /// Run analysis
    Run,

    /// Run tests
    Test
}

