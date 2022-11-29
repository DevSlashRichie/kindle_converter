#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    skc::execute().await;

    Ok(())
}