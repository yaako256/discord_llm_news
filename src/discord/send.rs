
use reqwest::header::HeaderMap;
use reqwest::blocking::Body;

use crate::discord::dtos;



/// discordに送信する関数
pub fn send_message(s: &str,webhook_url:String,errors: &mut Vec<String>) {

    let mut headers = HeaderMap::new();
    headers.append(
        "Content-Type",
        "application/json"
            .parse()
            .expect("application/jsonがパースできない"),
    );

    let message = dtos::Message::new(s);
    let message = serde_json::to_vec(&message).expect("jsonにserializeできません");
    let response = reqwest::blocking::Client::new()
        .post(webhook_url)
        .headers(headers)
        .body(Body::from(message))
        .send()
        .expect("エラー");
        
    if !response.status().is_success() {
        let msg = format!("discordがエラーを応答しました, {:?}", response.text());
        eprintln!("{}",msg);
        errors.push(msg);
    } 
}

