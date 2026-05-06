// RSS設定用構造体
use crate::models::news::NewsRss;

// 環境変数取得用
use dotenvy::dotenv;
use std::env;

// RSSから最新のニュースをいくつ扱うか
pub const RSS_GET_NUM: usize = 5;

// 1回のLLMリクエストで何回リトライするか
pub const MAX_RETRIES: usize = 5;

// サーバ負荷対策で何ミリ秒待つか
pub const SLEEP_TIME_MILLIS_RSS: u64 = 1000;
pub const SLEEP_TIME_MILLIS_BODY: u64 = 10000;

// Geminiに聞く時(ミス時)の待機時間
// 待ち時間はどうでもいいので適当に長めにとってる
pub const LLM_SLEEP_TIME: u64 = 31000;

// 指数バックオフで何倍ずつ増やすか
pub const BACKOFF_FACTOR: f64 = 1.3;
// 指数バックオフしたときの最大待機時間
pub const SLEEP_LLM_TIME_MAX: u64 = 120000;

// LLMに何秒の長考を許すか？
pub const LLM_THINK_TIME: u64 = 60;

/*
yahooのRSS[https://news.yahoo.co.jp/rss]の
カテゴリごとのニュースから取得。
トピックスは本文へのurlじゃなかった。
*/
pub fn get_news_config() -> Vec<NewsRss> {
    vec![
        NewsRss::new(
            100,
            "国内",
            "https://news.yahoo.co.jp/rss/categories/domestic.xml",
        ),
        NewsRss::new(
            200,
            "国際",
            "https://news.yahoo.co.jp/rss/categories/world.xml",
        ),
        NewsRss::new(
            300,
            "経済",
            "https://news.yahoo.co.jp/rss/categories/business.xml",
        ),
        NewsRss::new(
            400,
            "エンタメ",
            "https://news.yahoo.co.jp/rss/categories/entertainment.xml",
        ),
        NewsRss::new(
            500,
            "スポーツ",
            "https://news.yahoo.co.jp/rss/categories/sports.xml",
        ),
        NewsRss::new(600, "IT", "https://news.yahoo.co.jp/rss/categories/it.xml"),
        NewsRss::new(
            700,
            "科学",
            "https://news.yahoo.co.jp/rss/categories/science.xml",
        ),
        NewsRss::new(
            800,
            "ライフ",
            "https://news.yahoo.co.jp/rss/categories/life.xml",
        ),
        NewsRss::new(
            900,
            "地域",
            "https://news.yahoo.co.jp/rss/categories/local.xml",
        ),
    ]
}

// 環境変数の取得
pub struct Config {
    pub discord_webhook_url: String,
    pub gemini_api_key: String,
    pub gemini_model: String,
}
impl Config {
    pub fn from_env() -> Self {
        // envファイルから環境変数をロードする
        dotenv().ok();

        Self {
            discord_webhook_url: env::var("DISCORD_WEBHOOK_URL").expect("Missing"),
            gemini_api_key: env::var("GEMINI_API_KEY").expect("Missing"),
            gemini_model: env::var("GEMINI_MODEL").expect("Missing"),
        }
    }
}
