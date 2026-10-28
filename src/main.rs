use lib::server::run;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
