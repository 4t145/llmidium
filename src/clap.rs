use std::net::{IpAddr, SocketAddr};

use clap::Parser;

/// Command line arguments for the application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Host address to bind to
    #[arg(short, long, env, default_value = "[::]")]
    pub host: IpAddr,

    /// Port to listen on
    #[arg(short, long, env, default_value_t = 7777)]
    pub port: u16,

    /// Port to listen on
    #[arg(short, long, env, default_value_t = 300)]
    pub timeout: u32,
}

impl CliArgs {
    pub fn bind(&self) -> SocketAddr {
        SocketAddr::from((self.host, self.port))
    } 
}

/// Parse command line arguments
pub fn args() -> CliArgs {
    CliArgs::parse()
}
