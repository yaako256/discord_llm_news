


// RSS初期設定を受け取るための型
use discord_llm_news::models::news::NewsRss;
use discord_llm_news::discord::send;
use discord_llm_news::config;
use discord_llm_news::generate_news_summary;

fn main() {
    // 環境変数等
    let config = config::Config::from_env();

    // 扱うニュース一覧(コンフィグ)を取得
    let news_vec: Vec<NewsRss> = config::get_news_config();

    // エラーログを保持する変数
    let mut errors: Vec<String> = Vec::new();

    // RSSフィードとLLMを使って文章作成
    let res_text = generate_news_summary(&news_vec,&config,&mut errors);

    // 対象テキストをdiscordに送信してもらう。
    //send::send_message("いぇい",config.discord_webhook_url);
    send::send_message(&res_text,config.discord_webhook_url,&mut errors);
}
