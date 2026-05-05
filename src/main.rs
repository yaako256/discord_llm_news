


// RSS初期設定を受け取るための型
use discord_llm_news::models::news::NewsRss;
use discord_llm_news::discord::send;
use discord_llm_news::config;
use discord_llm_news::generate_news_summary;

fn main() {
    // 扱うニュース一覧(コンフィグ)を取得
    let news_vec: Vec<NewsRss> = config::get_news_config();

    // RSSフィードとLLMを使って文章作成
    let _text = generate_news_summary(&news_vec);


    // 環境変数等
    let config = config::Config::from_env();

     // 対象テキストをdiscordに送信してもらう。
    let aa =  send::send_message("いぇい",config.discord_webhook_url);
}
