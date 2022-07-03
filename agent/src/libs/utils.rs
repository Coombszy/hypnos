use reqwest::{Response, Url};

use crate::libs::structs::Args;


pub async fn server_check(config: Args) -> bool {

    let response = reqwest::Client::new()
        .get(Url::parse(&config.server).unwrap())
        .send()
        .await;

    match response {
        Ok(r) => {

        }
        Err(e) => { }
    }

    true
}

