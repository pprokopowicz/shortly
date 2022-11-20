mod app;

#[tokio::main]
async fn main() {
    app::start().await.expect("Failed to start the app");
}
