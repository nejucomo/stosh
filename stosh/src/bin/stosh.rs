#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    stosh::run().await
}
