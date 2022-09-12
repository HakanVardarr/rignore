#[tokio::main]
async fn main() {
    match rignore::run().await {
        Ok(_) => (),
        Err(e) => eprintln!("ERROR: {e}"),
    }
}
