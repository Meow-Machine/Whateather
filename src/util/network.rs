use std::time::Duration;
use colored::Colorize;
use reqwest::{Client, Response};
const RETRIES_TILL_FAILURE: u32 = 5;
const TIME_BETWEEN_TRIES: f32 = 5.0f32;
/// Retries getting information online, might be useful considering
/// unstable Internet connection
/// # Parameters:
/// - client &[`Client`] -> Internet client to be used
/// - url &[`std::str`] -> Url to be harvested
/// # Returns:
/// - Response
pub async fn retrying_get(client: &Client, url: &str) -> Response {
    let sleep_time = Duration::from_secs_f32(TIME_BETWEEN_TRIES);
    let mut tries = 0;

    // Retry until Failure state is reached
    while tries < RETRIES_TILL_FAILURE {
       let response = client.get(url).send().await;

        if response.is_ok() {
            return response.unwrap();
        }

        // Failure -> Wait and retry
        tries += 1;
        tokio::time::sleep(sleep_time).await;
    }

    panic!("{} couldn't get data from the Internet ({})", "[Failure]".yellow(), url)
}