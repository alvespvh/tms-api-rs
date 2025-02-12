pub mod database;
pub mod logger;
pub mod prelude;
pub mod settings;

use log::info;

#[tokio::main]
async fn main() {
    logger::init();
    info!("Starting the application");
    println!("Hello, world!");

    logger::telegram::notify("TESTE").await;
}
