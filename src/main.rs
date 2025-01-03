use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    zero2prod::run().await
}
