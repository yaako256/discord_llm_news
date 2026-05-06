// discord/send.rs

use crate::discord::dtos;

use reqwest::blocking::Client;
use std::time::Duration;

/// discordに送信する関数
pub fn send_message(s: &str, webhook_url: &String, errors: &mut Vec<String>) {
    // 構造体に
    let message = dtos::Message::new(s);

    // シリアライズ
    let message = match serde_json::to_vec(&message) {
        Ok(b) => b,
        Err(e) => {
            errors.push(format!("JSONシリアライズ失敗: {}", e));
            return;
        }
    };

    // タイムアウト設定付きクライアント
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build();

    let client = match client {
        Ok(c) => c,
        Err(e) => {
            errors.push(format!("Client作成失敗: {}", e));
            return;
        }
    };

    // 送信実行 (expectを排除)
    match client.post(webhook_url)
        .header("Content-Type", "application/json")
        .body(message)
        .send() 
    {
        Ok(response) => {
            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().unwrap_or_else(|_| "body不可読".into());
                errors.push(format!("Discord投稿失敗 [{}]: {}", status, body));
            }
        }
        Err(e) => {
            errors.push(format!("Discord通信失敗: {}", e));
        }
    }
}
