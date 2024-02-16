use color_eyre::{eyre::eyre, Report};
use reqwest::{Client, Url};
use serde_json::Value as JsValue;

/// this api will return a url of a picture of a duck
const API_URL: &str = "https://random-d.uk/api/v2/random";

#[tokio::main]
async fn main() -> Result<(), Report> {
    // client is in charge of actually calling the api. and any other https request
    let client = Client::new();

    let duck_image_url = random_duck_url(&client).await?;

    println!("{}", duck_image_url);

    return Ok(());
}


/// ths function makes a call to `https://random-d.uk/api/random` returning the url to the random duck image
/// # Errors:
/// - if the client fails to send the GET request
/// - if the response is not able to be parsed into a [serde_json::Value]
/// - if the `"url"` key is not present in the response, or it's value is not a valid [reqwest::Url]
async fn random_duck_url(client: &Client) -> Result<Url, Report> {
    // make an api request
    let api_response = client // using the client
        .get(API_URL) // call the api with GET
        .send().await? // send the request and wait for a response
        .text().await? // get the text from the response. this could take awhile so lets wait for it
        .parse::<JsValue>()?; // parse the text into a javascript object

    // extract the image_url from the response
    let image_url = api_response["url"] // look for a "url" key
        .as_str() // get the corresponding value if there is one
        .ok_or_else(|| {
            eyre!(
                "{} did not return a \"url\" key.\napi_response: {}",
                API_URL,
                api_response
            ) // if the value is not present set a corresponding error
        })? // convert [Option] to a [Result]
        .parse::<Url>()?; // parse the value into a [Url]

    return Ok(image_url);
}
