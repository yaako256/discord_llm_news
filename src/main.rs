// main.rs

// RSS初期設定を受け取るための型
use discord_llm_news::config;
use discord_llm_news::discord::send;
use discord_llm_news::generate_news_summary;
use discord_llm_news::models::news::NewsRss;

fn main() {
    // 環境変数等
    let config = config::Config::from_env();

    // 扱うニュース一覧(コンフィグ)を取得
    let news_vec: Vec<NewsRss> = config::get_news_config();

    // エラーログを保持する変数
    let mut errors: Vec<String> = Vec::new();

    // RSSフィードとLLMを使って文章作成
    let res_text = generate_news_summary(&news_vec, &config, &mut errors);

    // 対象テキストをdiscordに送信してもらう。
    if res_text != "" {
        send::send_message(&res_text, &config.discord_webhook_url, &mut errors);
    }

    // エラーがあったらそれもdiscordに送信してもらう。
    if !errors.is_empty() {
        let error_text = format!("## エラーログ\n```\n{}\n```", errors.join("\n"));
        // エラー送信自体が失敗する可能性を考慮し、ここではこれ以上 errors に push しない
        let mut send_errors = Vec::new(); 
        send::send_message(&error_text, &config.discord_webhook_url, &mut send_errors);
        
        if !send_errors.is_empty() {
            // Discord 送信すら失敗した場合は、最終手段として標準エラー出力に出して終了する
            // (eprintln! ではなくログライブラリ推奨だが、パニック回避のため)
            std::process::exit(1); 
        }
    }
}
