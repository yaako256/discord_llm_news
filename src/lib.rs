mod controller;
mod models;

mod config;

// RSSの初期設定用
use crate::config::get_news_config;

// LLMにリクエストするときの型
use crate::models::llm_request_fmt::{
    LLMRrequestFmtFirst,
    LLMRrequestFmtSecond, //LLMRrequestFmtFinal
};
// RSS初期設定を受け取るための型
use crate::models::news::{FeedItem, NewsRss};

// RSSの情報をURLから持ってくる関数
use crate::controller::fetch_feed;

// LLMに聞く関数
use crate::controller::llm_request;

// ログ出力用(デバッグ用)
use std::fs::File;
use std::io::Write;

pub fn generate_news_summary() -> String {
    // 扱うニュース一覧(コンフィグ)を取得
    let mut news_vec: Vec<NewsRss> = get_news_config();

    // エラーを保持しておくための変数
    //let mut errors = Vec::new();
    /*
       // 各ジャンルのタイトルとリンクを取得
       for news in &mut news_vec {
           let (feed_items, error) = fetch_feed::fetch_feed_items(news.rss_url.as_str());
           news.feed_items = feed_items;
           errors.push(error);
       }

       // LLMに聞くフォーマット(1回目)にあわせる
       let mut first_llm_request_vec = Vec::new();
       for news in &mut news_vec {
           for (i, f) in news.feed_items.iter_mut().enumerate() {
               f.id = news.id_start + (i as i16);
               first_llm_request_vec.push(LLMRrequestFmtFirst::new(
                   f.id,
                   news.genre.clone(),
                   f.title.clone(),
                   f.description.clone(),
               ));
           }
       }
    */
    // idとリンクを結びつけておく構造体
    let mut feed_items: Vec<FeedItem> = Vec::new();
    // LLMに聞くフォーマット(1回目)
    let mut first_llm_request_vec: Vec<LLMRrequestFmtFirst> = Vec::new();
    // エラーを保持しておくための変数
    let mut errors: Vec<String> = Vec::new();

    // RSSのリンクからFeeditemsを作り、idも振る
    // LLMに聞くフォーマット(1回目)も作り出す。
    for news in &mut news_vec {
        let (feed_item, first_llm_request) = fetch_feed::fetch_feed_items(
            news.id_start,
            news.genre.as_str(),
            news.rss_url.as_str(),
            &mut errors,
        );
        feed_items.extend(feed_item);
        first_llm_request_vec.extend(first_llm_request);

    }

    // デバッグ出力
    let mut file = File::create("logs/news_log.txt").expect("ファイル作成に失敗しました");
    for news in &feed_items {
        writeln!(file, "{:#?}", news).expect("ファイル書き込みに失敗しました");
    }

    // LLMに聞く(1回目:各ジャンル第3候補を選んでもらう)(候補idをリストを取得)
    // 仮
    let id_list = [
        100, 101, 102, 200, 201, 202, 300, 301, 302, 400, 401, 402, 500, 501, 502, 600, 601, 602,
        700, 701, 702, 800, 801, 802, 900, 901, 902,
    ];
    let list = llm_request::first_llm_request(first_llm_request_vec);

    // LLMに聞くフォーマット(2回目)にあわせる
    // ついでにこの中でニュースの本文を取得してる
    let second_llm_request_vec: Vec<LLMRrequestFmtSecond> =
        fetch_feed::filter_feed_items(&feed_items, &id_list, &mut errors);

    // LLMに聞く(2回目:各ジャンル第1候補を選んでもらう)(候補idをリストを取得)

    // LLMに聞くフォーマット(3回目)にあわせる。

    // LLMに実際に要約してもらい、本文を作成する。

    if (second_llm_request_vec.len() < 1) {
        println!("中身なくね？")
    }

    // デバッグ出力
    /*
    let mut file = File::create("logs/news_log.txt").expect("ファイル作成に失敗しました");
    for news in &second_llm_request_vec {
        writeln!(file, "{:#?}", news).expect("ファイル書き込みに失敗しました");
    }
 */
    // 2. Markdownファイル (純粋なテキストとして書き出す)
    let mut md_file = File::create("logs/news_md.md").expect("ファイル作成に失敗しました");
    // {:#?} をやめて、直接文字列を流し込む
    // 1つ目の記事の内容を「そのまま」出力します
    if let Some(first_news) = second_llm_request_vec.get(0) {
        // .write_all を使うか、writeln! なら "{}" を使う
        write!(md_file, "{}", first_news.contents.clone()).expect("ファイル書き込みに失敗しました");
        return first_news.contents.clone();
    }

    println!("{:#?}", errors);
    "".to_string()
}
