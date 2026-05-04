//use rss::Channel;
//use std::error::Error;

mod config;
mod models;

mod fetch_feed;
mod llm_request;

use crate::models::llm_request_fmt::{
    first_llm_request_fmt, second_llm_request_fmt,final_llm_request_fmt,
};
use crate::models::news::News;
use config::get_news_config;
use fetch_feed::fetch_feed_items;

use llm_request::{first_llm_request,second_llm_request,final_llm_request};


// ログ出力用(デバッグ用)
use std::fs::File;
use std::io::Write;

fn main() {
    // 扱うニュース一覧(コンフィグ)を取得
    let mut news_vec: Vec<News> = get_news_config();

    // エラーを保持しておくための変数
    let mut errors = Vec::new();

    // 各ジャンルのタイトルとリンクを取得
    for news in &mut news_vec {
        let (feed_items, error) = fetch_feed_items(news.rss_url.as_str());
        news.feed_items = feed_items;
        errors.push(error);
    }

    // LLMに聞くフォーマット(1回目)にあわせる
    let mut first_llm_request_vec = Vec::new();
    for news in &mut news_vec {
        for (i, f) in news.feed_items.iter().enumerate() {
            first_llm_request_vec.push(first_llm_request_fmt::new(
                news.id_start + (i as i16),
                news.genre.clone(),
                f.title.clone(),
            ));
        }
    }

    // LLMに聞く(1回目:各ジャンル第3候補を選んでもらう)
    let list = first_llm_request(first_llm_request_vec);


    /*
    // デバッグ出力
    let mut file = File::create("news_log.txt").expect("ファイル作成に失敗しました");
    for news in first_llm_request_vec {
        writeln!(file, "{:#?}", news).expect("ファイル書き込みに失敗しました");
    }
     */
}
