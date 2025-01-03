pub mod locator;
pub mod util;
mod temperature;
mod graphic;

use colored::Colorize;
use tokio::runtime::Runtime;

const USE_LOCATOR: &str = "IP";
fn main () {
    let runtime = Runtime::new().unwrap();
    let client = reqwest::Client::new();


    let location = runtime.block_on(async {
        match USE_LOCATOR {
            "IP" => {
                locator::locate_via_ip(&client).await
            },
            "GOOGLE" => {
                todo!("{} GOOGLE-API Location is not implemented yet!", "[TODO]".green());
            }
            "GPS" => {
                todo!("{} GPS Location is not implemented yet!", "[TODO]".green());
            }
            _ => {
                panic!("{} CANT FIND {} LOCATOR", "[Error]".red(), USE_LOCATOR);
            }
        }
    });


    let temperature = runtime.block_on(async {
        temperature::get_temperature(&location, &client).await
    });

    println!("{:?}", temperature);
}