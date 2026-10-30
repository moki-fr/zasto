use reqwest::blocking::Client;

pub fn check_api_key(api_key: &str) -> Result<bool, reqwest::Error> {
    let client = Client::new();

    let response = client
        .get("https://openrouter.ai/api/v1/auth/key")
        .bearer_auth(api_key)
        .send()?;

    Ok(response.status().is_success())
}
