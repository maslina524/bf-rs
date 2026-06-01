mod client;
mod events;
mod players;
mod json;

pub use client::{Client, ApiError, CrateError, BsResult};

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use tokio;

    #[tokio::test]
    async fn gamemodes_test() {
        dotenv().ok();

        let api_key = std::env::var("API_KEY").unwrap_or_default();
        let client = Client::new(api_key);
        let ret = client.events().gamemodes().await;
        println!("{ret:#?}");
    }

    #[tokio::test]
    async fn rotation_test() {
        dotenv().ok();

        let api_key = std::env::var("API_KEY").unwrap_or_default();
        let client = Client::new(api_key);
        let ret = client.events().rotation().await;
        println!("{ret:#?}");
    }
}