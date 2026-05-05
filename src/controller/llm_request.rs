use reqwest::blocking::Client;
//use reqwest::blocking::Body;
use serde::Serialize;
use serde_json::json;

use crate::models::llm_request_fmt::{
    LLMRrequestFmtFinal, LLMRrequestFmtFirst, LLMRrequestFmtSecond,
};

use crate::models::llm_dtos::GeminiIDResponse;

use crate::config::Config;

pub fn llm_request_first(fmt: &[LLMRrequestFmtFirst], config: &Config) -> Result<Vec<i16>, String> {
    // プロンプト作成
    let prompt = build_prompt(fmt,"**3つずつ**");

    // 実際にリクエスト
    let res_text = llm_request(prompt, config);

    serde_json::from_str::<GeminiIDResponse>(&res_text)
        .map(|parsed| parsed.id)
        .map_err(|e| format!("JSONパース失敗: {}", e))
}

pub fn llm_request_second(fmt: &[LLMRrequestFmtSecond], config: &Config) ->Result<Vec<i16>, String> {
    // データをJSON文字列に変換
    //let data_json = serde_json::to_string_pretty(fmt).unwrap_or_default();

    let prompt = build_prompt(fmt,"**1つずつ**");

    // 実際にリクエスト
    let res_text = llm_request(prompt, config);

    // i16のベクトルにパース
    serde_json::from_str::<GeminiIDResponse>(&res_text)
        .map(|parsed| parsed.id)
        .map_err(|e| format!("JSONパース失敗: {}", e))
}

pub fn llm_request_final(fmt: &Vec<LLMRrequestFmtFinal>, config: &Config) {
    // まだ未実装
}

// 実際にllm_requestするところ
fn llm_request(prompt: String, config: &Config) -> String {
    // geminiのエンドポイント(url)を作成
    let gemini_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/{}:generateContent?key={}",
        config.gemini_model, config.gemini_api_key
    );

    // クライアントのインスタンス
    // 本来再利用が望ましい
    let client = reqwest::blocking::Client::new();

    // Geminiに送るファイルを構築
    let body = json!({
        "contents": [{
            "parts": [{
                "text": prompt // プロンプト
            }]
        }],
        "generationConfig": {
            "response_mime_type": "application/json" // JSONモードを有効化
        }
    });

    // リクエストを送信
    let response = client.post(&gemini_url).json(&body).send();

    // 正常に返答が来たかの判別
    match response {
        Ok(res) if res.status().is_success() => {
            let res_body: serde_json::Value = res.json().unwrap_or_default();
            res_body["candidates"][0]["content"]["parts"][0]["text"]
                .as_str()
                .unwrap_or("")
                .to_string()
        }
        Ok(res) => {
            eprintln!("APIエラー: {}", res.status());
            "".to_string()
        }
        Err(e) => {
            eprintln!("通信エラー: {}", e);
            "".to_string()
        }
    }
}

// 共通のプロンプト生成ロジック
fn build_prompt<T: Serialize>(data: &[T], count_desc: &str) -> String {
    let data_json = serde_json::to_string_pretty(data).unwrap_or_default();
    format!(
        r#"あなたはプロのニュース編集者です。提供されたニュースデータ（JSON形式）を「面白さ」「話題性」「重要性」の観点から深く分析してください。

各ジャンルごとに、最も注目すべき記事を"{}"選出してください。

# 制約事項
1. 出力は必ず以下のJSON形式のみとし、**解説や挨拶は一切含めないでください**。
2. 選出した記事の id のみを配列に格納してください。
3. 重複する id は含めないでください

# 出力フォーマット
{{
  "id": [integer, integer, ...]
}}

## ニュースデータ
{}"#,
        count_desc, data_json
    )
}
