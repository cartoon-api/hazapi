mod conn;
mod app;

// External USEs
use tokio;

#[tokio::main]
async fn main() {
    app::run().await
        .expect("Failed to init the API")
}
