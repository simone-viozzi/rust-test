use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use reqwest::Client;
use std::io::{self, Write};
use log::{info, error};

pub async fn start_tui() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    loop {
        // Clear the screen and display the prompt
        io::stdout().execute(Clear(ClearType::All))?;
        print!("Enter your message (or 'quit' to exit): ");
        io::stdout().flush()?;

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            info!("User exited the TUI.");
            break;
        }

        info!("User entered: {}", input);

        // Call the API and print the response
        match client
            .get(&format!("http://127.0.0.1:3030/echo/{}", input))
            .send()
            .await
        {
            Ok(resp) => match resp.text().await {
                Ok(response) => {
                    info!("Received API response: {}", response);
                    println!("API Response: {}", response);
                }
                Err(e) => {
                    error!("Failed to parse API response: {}", e);
                }
            },
            Err(e) => {
                error!("Failed to send request to API: {}", e);
            }
        }

        // Pause before the next iteration
        io::stdout().execute(Clear(ClearType::FromCursorDown))?;
    }

    Ok(())
}
