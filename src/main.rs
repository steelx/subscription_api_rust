use subscription_api_rust::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
