use std::time::Duration;
use color_eyre::eyre::Ok;
use thirtyfour::prelude::*;
use url::Url;

#[tokio::main]
async fn main() {
    // Set up WebDriver session
    color_eyre::install()?;
    let caps = DesiredCapabilities::firefox();
    let session = WebDriver::new("http://localhost:4444", caps).await?;
    
    // Step 1: Visit the authentication URL and get the code
    let auth_url = "https://auth.gog.com/auth?client_id=46899977096215655&redirect_uri=https%3A%2F%2Fembed.gog.com%2Fon_login_success%3Forigin%3Dclient&response_type=code&layout=client2";
    session.goto(auth_url).await?;
    // Let the user log in manually or perform any necessary actions
    // This is where the user might need to enter credentials
    let elem_form = session.find(By::Id("login_login")).await?;
    elem_form.wait_until().not_enabled().await?;
    // Wait for the user to finish authentication (you might need to adjust the time)
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Get the current URL, which includes the code in the query parameters
    let current_url = session.current_url().await?;
    
    // Extract the code from the URLcargo run --example tokio_async

    let url_str: String = current_url.to_string();
    let url = Url::parse(&url_str).expect("Failed to parse URL");
        let url_parts: Vec<&str> = url_str.split('?').collect();
    let query_params: Vec<&str> = url_parts[1].split('&').collect();
    let mut code = "";
    for param in query_params {
        let key_value: Vec<&str> = param.split('=').collect();
        if key_value[0] == "code" {
            code = key_value[1];
            break;
        }
    }
    // Step 2: Use the obtained code to get the token
    let token_url = "https://auth.gog.com/token";
    let client_id = "46899977096215655"; // Replace with your actual client ID
    let client_secret = "9d85c43b1482497dbbce61f6e4aa173a433796eeae2ca8c5f6129f2dc4de46d9"; // Replace with your actual client secret

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", "https%3A%2F%2Fembed.gog.com%2Fon_login_success%3Forigin%3Dclient")
    ];

    let response = reqwest::Client::new().post(token_url).form(&params).send().await?;
    let token_json: String = response.text().await?;

    // Now you have the token_json, and you can extract the access token
    // let access_token = token_json["access_token"].as_str().unwrap_or_default();

    println!("Access Token: {}", token_json);

    // Close the WebDriver session
    session.quit().await?;

    Ok(())
}
