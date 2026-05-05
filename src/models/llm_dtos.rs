use serde::Deserialize;


// idだけを保持する
// 1回と2回目のリクエストで使う。
#[derive(Debug,Deserialize)]
pub struct GeminiIDResponse {
    pub id: Vec<i16>,
}