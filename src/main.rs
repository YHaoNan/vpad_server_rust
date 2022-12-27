mod server;
mod message;
mod constants;

#[tokio::main]
async fn main() -> server::Result {
    let vpad_server = server::VPadServer::bind("0.0.0.0:1236");
    vpad_server.start().await?;
    Ok(())
}