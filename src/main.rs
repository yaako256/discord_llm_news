mod controller;
mod models;
mod config;
mod discord;

use config::Config;
use discord::send;

fn main() {
    // RSSフィードとLLMを使って文章作成
    //let _text = discord_llm_news::generate_news_summary();


    // 環境変数等
    let config = Config::from_env();

     // 対象テキストをdiscordに送信してもらう。
    let aa =  send::send_message("いぇい",config.discord_webhook_url);
}
