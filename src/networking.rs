use std::error::Error;
use reqwest::Client;
use serde_json::Value;
use crate::dotenv_tools::{ROCK_ENDPOINT, ROCK_TOKEN};
use crate::handler::QMessage;
use crate::qsegment_constructor::{QSegmentConstructor, Types};

pub async fn send_get_request(url: &str) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?.json::<Value>().await?;
    Ok(response)
}
pub async fn send_message(message: &str) -> Result<(), Box<dyn Error>> {
    let rock_endpoint = &format!("http://{}/send_group_msg?access_token={}", *ROCK_ENDPOINT, *ROCK_TOKEN);
    println!("{}", rock_endpoint);

    let segment = QSegmentConstructor::create(Types::Plain, message);
    let result = QSegmentConstructor::factory(vec![segment]);

    let structbody = QMessage{
        group_id: 946085440,
        message: result,
        auto_escape: false,
    };

    let client = Client::new();
    client
        .post(rock_endpoint)
        .body(serde_json::to_string(&structbody)?)
        .send()
        .await?;

    Ok(())
}
