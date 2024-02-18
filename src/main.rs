use color_eyre::{eyre::eyre, Report};
use reqwest::{Client, Url};
use serde_json::Value as JsonValue;

/// this api will return a url of a picture of a duck
const RANDOM_DUCK_API_URL: &str = "https://random-d.uk/api/v2/random";

#[tokio::main]
async fn main() -> Result<(), Report> {
    // client is in charge of actually calling the api. and any other https request
    let client = Client::new();

    let duck_image_url = random_duck_url(&client).await?;

    println!("{}", duck_image_url);

    return Ok(());
}

/// ths function makes a `GET` call to `api_url` returning the response as a JsonValue
/// # Errors:
/// - if the client fails to send the GET request
/// - if the response is not able to be parsed into a [serde_json::Value]
async fn get_json(client: &Client, api_url: &str) -> Result<JsonValue, Report> {
    let json_response = client // using the client
        .get(api_url) // setup the GET API request
        .send().await? // send the request and wait for a response
        .text().await? // extract the text from the response. this could take awhile so lets wait for it
        .parse::<JsonValue>()?; // parse the text into a [JsonValue]

    return Ok(json_response);
}

/// This function checks for a `"url"` key, returning its value
/// # Errors:
/// - if the `"url"` key is not present in the response, or it's value is not a valid [reqwest::Url]
async fn extract_url(value: JsonValue) -> Result<Url, Report> {
    let url = value["url"] // look for a "url" key
        .as_str() // get the corresponding value if there is one
        .ok_or_else(|| {
            eyre!(
                "The json value did not contain a \"url\" key.\n{}",
                value
            ) // if the value is not present set a corresponding error
        })? // convert [Option] to a [Result]
        .parse::<Url>()?; // parse the value into a [Url]

    return Ok(url);
}

/// ths function makes a call to [RANDOM_DUCK_API_URL] returning the url to the random duck image
/// # Errors:
/// - if the client fails to send the GET request
/// - if the response is not able to be parsed into a [serde_json::Value]
/// - if the `"url"` key is not present in the response, or it's value is not a valid [reqwest::Url]
async fn random_duck_url(client: &Client) -> Result<Url, Report> {
    // make an api request
    let api_response = get_json(client, RANDOM_DUCK_API_URL).await?;

    // extract the image_url from the response
    let image_url = extract_url(api_response).await?;

    return Ok(image_url);
}
