// Test program to verify API parsing works
use donetick_tui::api::{ApiClient, Chore};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let url = std::env::var("DONETICK_URL").expect("DONETICK_URL not set");
    let token = std::env::var("DONETICK_TOKEN").expect("DONETICK_TOKEN not set");

    println!("Connecting to: {}", url);

    let client = match ApiClient::new(url, token) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to create client: {}", e);
            return;
        }
    };

    println!("Fetching chores...");

    match client.list_chores().await {
        Ok(chores) => {
            println!("✓ SUCCESS! Loaded {} chores:", chores.len());
            for chore in chores {
                println!("  - {} (id: {}, status: {:?})", chore.name, chore.id, chore.status);
            }
        }
        Err(e) => {
            println!("✗ FAILED: {}", e);
        }
    }
}
