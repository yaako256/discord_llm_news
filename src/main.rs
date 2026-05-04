//use rss::Channel;
//use std::error::Error;

mod config;
mod models;

mod fetch_feed;
mod llm_request;

use crate::models::llm_request_fmt::{
    final_llm_request_fmt, first_llm_request_fmt, second_llm_request_fmt,
};
use crate::models::news::NewsRss;
use config::get_news_config;
use fetch_feed::fetch_feed_items;
use llm_request::{final_llm_request, first_llm_request, second_llm_request};

// ログ出力用(デバッグ用)
use std::fs::File;
use std::io::Write;



fn main() {
    // 扱うニュース一覧(コンフィグ)を取得
    let mut news_vec: Vec<NewsRss> = get_news_config();

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
        for (i, f) in news.feed_items.iter_mut().enumerate() {
            f.id = news.id_start + (i as i16);
            first_llm_request_vec.push(first_llm_request_fmt::new(
                f.id,
                news.genre.clone(),
                f.title.clone(),
                f.description.clone(),
            ));
        }
    }

    // LLMに聞く(1回目:各ジャンル第3候補を選んでもらう)(候補idをリストを取得)
    // 仮
    let id_list = [
        100, 101, 102, 200, 201, 202, 300, 301, 302, 400, 401, 402, 500, 501, 502, 600, 601, 602,
        700, 701, 702, 800, 801, 802, 900, 901, 902,
    ];
    //let list = first_llm_request(first_llm_request_vec);

    // LLMに聞くフォーマット(2回目)にあわせる
    // ついでにこの中でニュースの本文を取得してる
    let second_llm_request_vec = fetch_feed::filter_feed_items(&news_vec, &id_list);

    // LLMに聞く(2回目:各ジャンル第1候補を選んでもらう)(候補idをリストを取得)
    

    // LLMに聞くフォーマット(3回目)にあわせる。

    // LLMに実際に要約してもらい、本文を作成する。

    // 対象テキストをdiscordに送信してもらう。





    // デバッグ出力
    let mut file = File::create("news_log.txt").expect("ファイル作成に失敗しました");
    for news in &second_llm_request_vec {
        writeln!(file, "{:#?}", news).expect("ファイル書き込みに失敗しました");
    }

    // 2. Markdownファイル (純粋なテキストとして書き出す)
    let mut md_file = File::create("news_md.md").expect("ファイル作成に失敗しました");
    // {:#?} をやめて、直接文字列を流し込む
    // 1つ目の記事の内容を「そのまま」出力します
    if let Some(first_news) = second_llm_request_vec.get(0) {
        // .write_all を使うか、writeln! なら "{}" を使う
        write!(md_file, "{}", first_news.contents).expect("ファイル書き込みに失敗しました");
    }
}
