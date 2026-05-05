use serde::Serialize;
use serde_json::json;

use crate::models::llm_request_fmt::{
    LLMRrequestFmtFinal, LLMRrequestFmtFirst, LLMRrequestFmtSecond,
};

use crate::models::llm_dtos::{GeminiIDResponse, GeminiTextResponse};

use crate::{config, config::Config};

// 日付取得用
use time::OffsetDateTime;

pub fn llm_request_first(
    fmt: &[LLMRrequestFmtFirst],
    config: &Config,
    errors: &mut Vec<String>,
) -> Result<Vec<i16>, String> {
    // プロンプト作成
    let prompt = build_prompt(fmt, "**3つずつ**");

    // 実際にリクエスト
    let res_text = llm_request(prompt, config, "first", errors);

    serde_json::from_str::<GeminiIDResponse>(&res_text)
        .map(|parsed| parsed.id)
        .map_err(|_| {
            let msg = format!("JSONパース失敗 (llm_request_first)");
            errors.push(msg.clone());
            msg
        })
}

pub fn llm_request_second(
    fmt: &[LLMRrequestFmtSecond],
    config: &Config,
    errors: &mut Vec<String>,
) -> Result<Vec<i16>, String> {
    // データをJSON文字列に変換
    //let data_json = serde_json::to_string_pretty(fmt).unwrap_or_default();

    let prompt = build_prompt(fmt, "**1つずつ**");

    // 実際にリクエスト
    let res_text = llm_request(prompt, config, "Second", errors);

    // i16のベクトルにパース
    serde_json::from_str::<GeminiIDResponse>(&res_text)
        .map(|parsed| parsed.id)
        .map_err(|_| {
            let msg = format!("JSONパース失敗 (llm_request_second)");
            errors.push(msg.clone());
            msg
        })
}

pub fn llm_request_final(
    fmt: &[LLMRrequestFmtFinal],
    config: &Config,
    errors: &mut Vec<String>,
) -> Result<String, String> {
    // jsonにシリアライズ
    let data_json = serde_json::to_string_pretty(fmt).unwrap_or_default();

    let date_string = get_date_string();
    // プロンプト作成
    let prompt = format!(
        r#"あなたは優秀なニュースキャスター兼エディターです。
渡されたJSON形式のニュースリストを元に、Discord投稿用の「今日のニュースまとめ」を作成してください。
また、本日は**{}**です。

# 制約
- 出力は必ず指定のJSON形式を守ること。
- 文字数はDiscordの制限(2000文字)を考慮し、1800文字以内。
- Markdown記法（# や ##、** 等）を活用すること。
- 絵文字等を使い、キャッチーに仕上げること。


# 構成ルール
1. **挨拶と日付**: 冒頭に「📅 2026年5月5日(火)のニュース」のように記載。
2. **ニュース本文**: 
   - ジャンルごとに整理（例：IT、スポーツ、エンタメ）。
   - タイトルは元のままではなく、思わずクリックしたくなるようなキャッチーなものに書き換える。
   - 要約は3行程度で、適切な絵文字を各所に散りばめる。
3. **本日の豆知識**: 
   - ニュースとは別に、言葉の由来や面白い雑学を1つ紹介。
4. **締めの一言**: 
   - 読者が一日を元気に始められるような温かい励まし。


# 構成サンプル
```
# 📅 20XX年X月X日(火)のニュース
## エンタメ
### いい感じにしたタイトル
本文要約

## スポーツ
### いい感じにしたタイトル
本文要約

...最後まで...

## 本日の豆知識！
実は名前に使っていい漢字と使っちゃいけない漢字って定義されてるらしいよ。(実際はもっと長く)

本日も頑張ってください！(本日の励ましの言葉)
```

# 出力フォーマット
{{
  "contents": "（ここにすべての文章をMarkdown形式で入れる）"
}}

# ニュースデータ
```json
{}
```"#,
      date_string,  data_json
    );

    // 実際にリクエスト
    let res_text = llm_request(prompt, config, "final", errors);

    // Stringにパース
    serde_json::from_str::<GeminiTextResponse>(&res_text)
        .map(|parsed| parsed.contents)
        .map_err(|_| {
            let msg = format!("JSONパース失敗 (llm_request_final)");
            errors.push(msg.clone());
            msg
        })
}

// 実際にllm_requestするところ
fn llm_request(prompt: String, config: &Config, time: &str, errors: &mut Vec<String>) -> String {
    // geminiのエンドポイント(url)を作成
    let gemini_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/{}:generateContent?key={}",
        config.gemini_model, config.gemini_api_key
    );

    // クライアントのインスタンス
    // 本来再利用が望ましい
    //let client = reqwest::blocking::Client::new();
    // 長考を認める(秒)
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(config::LLM_THINK_TIME))
        .build()
        .unwrap();

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
            let msg = format!("APIエラー [status:{}] ({})", res.status(), time);
            errors.push(msg);
            "".to_string()
        }
        Err(_) => {
            let msg = format!("通信エラー{}", time);
            errors.push(msg);
            "".to_string()
        }
    }
}

// 共通のプロンプト生成ロジック
fn build_prompt<T: Serialize>(data: &[T], count_desc: &str) -> String {
    // jsonにシリアライズ
    let data_json = serde_json::to_string_pretty(data).unwrap_or_default();

    // プロンプトを作成
    format!(
        r#"あなたはプロのニュース編集者です。提供されたニュースデータ（JSON形式）を「面白さ」「話題性」「重要性」の観点から深く分析してください。

各ジャンルごとに、最も注目すべき記事を"{}"選出してください。

# 制約事項
1. 出力は必ず以下のJSON形式のみとし、**解説や挨拶は一切含めないでください**。
2. 選出した記事の **idのみ**を配列に格納し、**出力フォーマットを厳守してください**。
3. 重複する id は含めないでください

# 出力フォーマット
{{
  "id": [integer, integer, ...]
}}

## ニュースデータ
```json
{}
```"#,
        count_desc, data_json
    )
}


fn get_date_string() -> String {
    // 現在時刻を取得（ローカルが取れなければUTC）
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

    // 曜日の取得と日本語変換
    let weekday_jp = match now.weekday() {
        time::Weekday::Monday    => "月",
        time::Weekday::Tuesday   => "火",
        time::Weekday::Wednesday => "水",
        time::Weekday::Thursday  => "木",
        time::Weekday::Friday    => "金",
        time::Weekday::Saturday  => "土",
        time::Weekday::Sunday    => "日",
    };

    // フォーマットしてStringにする
    // 5月6日のように「0」を消したい場合は padding:none を指定
    let date_str = format!(
        "{}年{}月{}日({})",
        now.year(),
        now.month() as u8, // Month型を数値(u8)にキャスト
        now.day(),
        weekday_jp
    );

    // 例:"2026年5月6日(水)"
    date_str
}