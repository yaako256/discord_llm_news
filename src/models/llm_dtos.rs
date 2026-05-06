// models/llm_dtos.rs

// シリアライズ用
use serde::Deserialize;

// idだけを保持する
// 1回と2回目のリクエストで使う。
#[derive(Debug,Deserialize)]
pub struct GeminiIDResponse {
    pub id: Vec<i16>,
}

// テキストだけを保持する
// 最後のリクエストで使う

// idだけを保持する
// 1回と2回目のリクエストで使う。
#[derive(Debug,Deserialize)]
pub struct GeminiTextResponse {
    pub contents: String,
}