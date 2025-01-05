pub mod locator;
pub mod util;
mod temperature;

use colored::Colorize;
use tokio::runtime::Runtime;

const USE_LOCATOR: &str = "IP";
fn main () {
    let runtime = Runtime::new().unwrap();
    let client = reqwest::Client::new();


    // let location = runtime.block_on(async {
    //     locator::locate(USE_LOCATOR, &client).await
    // });


    let temperature = runtime.block_on(async {
        let location = locator::locate(USE_LOCATOR, &client).await;
        temperature::get_temperature(&location, &client).await
    });

    println!("{:?}", temperature);
}
