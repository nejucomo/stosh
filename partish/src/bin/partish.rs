#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    partish::run().await
}
