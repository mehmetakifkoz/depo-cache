use futures::StreamExt;
use chromiumoxide::browser::{Browser, BrowserConfig};
use std::io::{self, Write};
use regex::Regex;
use std::fs::File;
use std::collections::HashMap;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Launch browser in visible mode
    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    // Spawn event handler
    let _handle = async_std::task::spawn(async move {
        while let Some(_event) = handler.next().await {
            // just handle events, do nothing
        }
    });

    // Open the desired page
    let _page = browser.new_page("https://www.farmazonrx.com.tr").await?;

    // Menu loop
    loop {
        println!("\n--- Script Menu ---");
        println!("1. Start");
        println!("2. Stop");
        print!("Select an option: ");
        io::stdout().flush()?; // Make sure the prompt shows

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Start selected (empty for now)...");
                // TODO: Add your script actions here
                    let mut barcodes: HashMap<String, String> = HashMap::new();
                    let re = Regex::new(r"\b\d{13}\b")?;

                    // Loop through a fixed range of IDs
                    for id in 1..11111 {
                        println!("Processing product ID: {}", id);

                        // Open the product page
                        let current_page = browser
                            .new_page(format!("https://www.farmazonrx.com.tr/product/{}", id))
                            .await?;

                        // Close all other pages except current
                        let pages = browser.pages().await?;
                        let current_target_id = current_page.target_id().clone();
                        for page in pages {
                            if *page.target_id() != current_target_id {
                                page.close().await?;
                            }
                        }

                        // Wait a little for page to load
                        async_std::task::sleep(std::time::Duration::from_secs(3)).await;

                        // Get page HTML
                        let html = current_page.content().await?;

                        // Try to find a 13-digit barcode
                        if let Some(cap) = re.captures(&html) {
                            let barcode = &cap[0];
                            println!("Found barcode: {}", barcode);
                            // Save to dictionary
                            barcodes.insert(barcode.to_string(), id.to_string());
                            // Save all barcodes to farmazonrx.json 
                            let mut file = File::create("farmazonrx.json")?;
                            file.write_all(serde_json::to_string_pretty(&barcodes)?.as_bytes())?;
                            println!("All barcodes saved to farmazonrx.json");
                        } else {
                            println!("No barcode found for ID: {}", id);
                        }
                    }
            }

            "2" => {
                println!("Stopping program...");    
                break; // exit the loop naturally
            }
            _ => println!("Invalid option, please try again."),
        }
    }

    // Clean exit
    Ok(())
}
