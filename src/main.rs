mod api_client;
mod error;
mod resolve;
mod server;
mod tools;

use clap::Parser;
use rmcp::ServiceExt;

#[derive(Parser)]
#[command(name = "mcp-icon-visual")]
struct Cli {
    #[arg(long, default_value = "https://icons.buan.me")]
    api_base: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let server = server::IconServer::new(cli.api_base)
        .serve(rmcp::transport::stdio())
        .await?;
    server.waiting().await?;
    Ok(())
}
